//! Connection pool management via deadpool-postgres.
//!
//! Production builds a [`Database`] via [`Database::new`] with settings read
//! from env vars (see [`DbSettings::from_env`]). Tests build one via the same
//! [`Database::new`] with a [`DbSettings::for_test`] pointed at a testcontainer
//! Postgres — both paths go through the SAME pool builder so any constraint
//! (e.g. deadpool's `Runtime::Tokio1` requirement when timeouts are set)
//! applies equally and a production-only bug in pool construction is
//! impossible.

use deadpool_postgres::{Manager, Pool, Runtime};
use std::env;
use std::time::Duration;
use tokio::sync::OnceCell;
use tokio_postgres::{Config, NoTls};
use tracing::{debug, instrument, trace};

use super::state::{AtomicDatabaseState, DatabaseState};
use crate::error::DbError;

/// Global lazy-initialized database instance.
static DATABASE: OnceCell<Database> = OnceCell::const_new();

/// Pre-init state observable before `DATABASE` is populated. Set to `Failed`
/// when `Database::new`'s caller (currently only [`get_db`]) sees an error
/// path so [`get_state`] can report a terminal failure even though `OnceCell`
/// itself stays empty (and will retry on the next call).
pub(super) static INIT_STATE: AtomicDatabaseState =
    AtomicDatabaseState::new(DatabaseState::AwaitConnect);

/// Connection settings consumed by [`Database::new`]. Production reads via
/// [`Self::from_env`]; tests construct directly with [`Self::for_test`].
#[derive(Debug, Clone)]
pub struct DbSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub pool_size: usize,
}

impl DbSettings {
    /// Read settings from `DATABASE_*` env vars, falling back to local-Postgres
    /// defaults. The defaults match `database/compose.yaml`.
    #[must_use]
    pub fn from_env() -> Self {
        // 127.0.0.1 not "localhost": Proton may not resolve localhost.
        Self {
            host: env::var("DATABASE_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env::var("DATABASE_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(55432),
            user: env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".into()),
            password: env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "changeit".into()),
            dbname: env::var("DATABASE_NAME").unwrap_or_else(|_| "postgres".into()),
            pool_size: env::var("DATABASE_POOL_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(16),
        }
    }

    /// Settings for an ephemeral testcontainer Postgres. `password = "postgres"`
    /// matches the `testcontainers_modules::postgres::Postgres` default, and
    /// `pool_size = 4` is enough for any single integration test.
    #[cfg(test)]
    #[must_use]
    pub fn for_test(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            user: "postgres".into(),
            password: "postgres".into(),
            dbname: "postgres".into(),
            pool_size: 4,
        }
    }
}

pub struct Database {
    pool: Pool,
    state: AtomicDatabaseState,
}

impl Database {
    /// Builds a Database from explicit settings. **Single pool-builder site for
    /// both production and tests** — any deadpool constraint (e.g. the
    /// `Runtime::Tokio1` requirement that comes with `wait_timeout` /
    /// `create_timeout`) is exercised by both paths. Tests catch any
    /// production-only bug in pool construction.
    ///
    /// # Panics
    /// Panics if `Pool::builder(...).build()` fails. This is a programmer error
    /// (misconfigured timeouts/runtime); the panic hook installed in
    /// `lib.rs::init` re-emits it through `tracing::error!` so it doesn't die
    /// silently inside a spawned task.
    #[instrument(level = "trace", name = "database_new", skip_all, fields(host = %settings.host, port = settings.port, user = %settings.user, dbname = %settings.dbname, pool_size = settings.pool_size))]
    #[must_use]
    pub fn new(settings: &DbSettings) -> Self {
        trace!("constructing database pool");

        let mut cfg = Config::new();
        cfg.host(&settings.host);
        cfg.port(settings.port);
        cfg.user(&settings.user);
        cfg.password(&settings.password);
        cfg.dbname(&settings.dbname);

        trace!("constructing Manager");
        let manager = Manager::new(cfg, NoTls);
        // Bound the failure-mode latency: without these timeouts, get_conn()
        // can hang for tens of seconds on a TCP handshake to a vanished host,
        // stretching watchdog ticks during a DB outage. 5s lets a single tick
        // surface "DB unreachable" quickly and retry on the next interval.
        trace!("calling Pool::builder().build()");
        // `Runtime::Tokio1` is REQUIRED here. deadpool 0.12 demands an explicit
        // runtime declaration whenever wait_timeout / create_timeout are set;
        // without it `.build()` returns `BuildError::NoRuntimeSpecified` even
        // though we're already running on a Tokio runtime. Handle::current()
        // is NOT consulted.
        let pool = match Pool::builder(manager)
            .runtime(Runtime::Tokio1)
            .max_size(settings.pool_size)
            .wait_timeout(Some(Duration::from_secs(5)))
            .create_timeout(Some(Duration::from_secs(5)))
            .build()
        {
            Ok(p) => {
                trace!("Pool::builder().build() returned Ok");
                p
            }
            Err(e) => {
                // Was previously .expect(); panicking inside an async-spawned task
                // dies silently and leaves the OnceCell empty (callback never
                // fires, SQF never sees TransientFailure). Bail more loudly.
                tracing::error!(error = %e, "Pool::builder().build() failed");
                panic!("Failed to build postgres pool: {e}");
            }
        };

        trace!("pool built; lazy connections will be established on first get_conn");

        Self {
            pool,
            state: AtomicDatabaseState::new(DatabaseState::ConnectedAwaitInit),
        }
    }

    /// Gets a pooled Postgres client.
    ///
    /// # Errors
    /// Returns a pool error if no client can be checked out from the pool.
    #[instrument(level = "trace", name = "database_get_conn", skip_all)]
    pub async fn get_conn(
        &self,
    ) -> Result<deadpool_postgres::Client, deadpool_postgres::PoolError> {
        let status = self.pool.status();
        trace!(
            available = status.available,
            size = status.size,
            max_size = status.max_size,
            "requesting client from pool"
        );
        let result = self.pool.get().await;
        match &result {
            Ok(_) => trace!("client checked out"),
            Err(e) => trace!(error = %e, "pool.get failed"),
        }
        result
    }

    pub fn state(&self) -> DatabaseState {
        if self.pool.is_closed() {
            return DatabaseState::AwaitConnect;
        }

        let status = self.pool.status();
        if status.available == 0 && status.size == 0 {
            return DatabaseState::AwaitConnect;
        }

        self.state.load()
    }

    pub fn set_state(&self, state: DatabaseState) {
        self.state.store(state);
    }
}

/// Gets the lazily initialized database handle.
///
/// # Errors
/// Returns a `tokio_postgres::Error` if the database configuration is invalid.
#[instrument(level = "trace", name = "database_get_db", skip_all)]
pub async fn get_db() -> Result<&'static Database, tokio_postgres::Error> {
    let was_inited = DATABASE.initialized();
    trace!(already_inited = was_inited, "get_db called");
    match DATABASE
        .get_or_try_init(|| async {
            trace!("initializing DATABASE OnceCell");
            let db = Database::new(&DbSettings::from_env());
            trace!("Database::new returned, storing in OnceCell");
            Ok(db)
        })
        .await
    {
        Ok(db) => {
            trace!("get_db returning database handle");
            Ok(db)
        }
        Err(e) => {
            trace!(error = %e, "get_db init failed; setting INIT_STATE=Failed");
            INIT_STATE.store(DatabaseState::Failed);
            Err(e)
        }
    }
}

/// Gets a pooled database client.
///
/// # Errors
/// Returns an init error if the database cannot be initialized, or a pool
/// error if a client cannot be checked out.
#[instrument(level = "trace", name = "database_get_client", skip_all)]
pub async fn get_client() -> Result<deadpool_postgres::Client, DbError> {
    let db = get_db().await.map_err(DbError::Init)?;
    db.get_conn().await.map_err(DbError::Pool)
}

pub fn get_state() -> DatabaseState {
    match DATABASE.get() {
        Some(db) => db.state(),
        None => INIT_STATE.load(),
    }
}

/// Arma-callable: returns the current database state.
#[instrument(level = "debug", name = "get_database_state", skip_all)]
pub fn get_database_state() -> DatabaseState {
    let state = get_state();
    debug!(?state, "database state requested");
    state
}

/// Spins up an ephemeral Postgres testcontainer and a [`Database`] pointed at
/// it via the **same** [`Database::new`] used in production. Test modules
/// (database/certification/ranks) MUST use this — building a [`Pool`] directly
/// in test code lets production-only pool-builder bugs (the
/// `Runtime::Tokio1` / timeouts saga) slip past `cargo test`.
///
/// Returns `(container, database)`. Keep the container handle alive for the
/// duration of the test — dropping it stops the container.
#[cfg(test)]
pub(crate) async fn start_test_db() -> (
    testcontainers_modules::testcontainers::ContainerAsync<
        testcontainers_modules::postgres::Postgres,
    >,
    Database,
) {
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers::ImageExt;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;

    let container = Postgres::default()
        .with_tag("18-alpine")
        .start()
        .await
        .expect("failed to start postgres container");
    let host = container
        .get_host()
        .await
        .expect("failed to get host")
        .to_string();
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("failed to get port");

    let db = Database::new(&DbSettings::for_test(host, port));
    (container, db)
}
