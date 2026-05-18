//! Connection pool management via deadpool-postgres.

use deadpool_postgres::{Manager, Pool};
use std::env;
use std::time::Duration;
use tokio::sync::OnceCell;
use tokio_postgres::{Config, NoTls};
use tracing::debug;

use super::state::{AtomicDatabaseState, DatabaseState};
use crate::error::DbError;

/// Global lazy-initialized database instance.
static DATABASE: OnceCell<Database> = OnceCell::const_new();

/// Pre-init state observable before `DATABASE` is populated. Set to `Failed`
/// when `init_from_env` errors so [`get_state`] can report a terminal failure
/// even though `OnceCell` itself stays empty (and will retry on the next call).
pub(super) static INIT_STATE: AtomicDatabaseState =
    AtomicDatabaseState::new(DatabaseState::AwaitConnect);

pub struct Database {
    pool: Pool,
    state: AtomicDatabaseState,
}

impl Database {
    fn init_from_env() -> Self {
        let mut cfg = Config::new();

        // 127.0.0.1 not "localhost": Proton may not resolve localhost.
        cfg.host(env::var("DATABASE_HOST").unwrap_or_else(|_| "127.0.0.1".into()));
        cfg.port(
            env::var("DATABASE_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(55432),
        );
        cfg.user(env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".into()));
        cfg.password(env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "changeit".into()));
        cfg.dbname(env::var("DATABASE_NAME").unwrap_or_else(|_| "postgres".into()));

        let pool_size = env::var("DATABASE_POOL_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(16);

        let manager = Manager::new(cfg, NoTls);
        // Bound the failure-mode latency: without these timeouts, get_conn()
        // can hang for tens of seconds on a TCP handshake to a vanished host,
        // stretching watchdog ticks during a DB outage. 5s lets a single tick
        // surface "DB unreachable" quickly and retry on the next interval.
        let pool = Pool::builder(manager)
            .max_size(pool_size)
            .wait_timeout(Some(Duration::from_secs(5)))
            .create_timeout(Some(Duration::from_secs(5)))
            .build()
            .expect("Failed to build postgres pool");

        Self {
            pool,
            state: AtomicDatabaseState::new(DatabaseState::ConnectedAwaitInit),
        }
    }

    /// Gets a pooled Postgres client.
    ///
    /// # Errors
    /// Returns a pool error if no client can be checked out from the pool.
    pub async fn get_conn(
        &self,
    ) -> Result<deadpool_postgres::Client, deadpool_postgres::PoolError> {
        self.pool.get().await
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
pub async fn get_db() -> Result<&'static Database, tokio_postgres::Error> {
    match DATABASE
        .get_or_try_init(|| async { Ok(Database::init_from_env()) })
        .await
    {
        Ok(db) => Ok(db),
        Err(e) => {
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
pub fn get_database_state() -> DatabaseState {
    let state = get_state();
    debug!(?state, "database state requested");
    state
}
