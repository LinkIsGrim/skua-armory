//! Database schema bootstrap operations.

use std::sync::LazyLock;

use arma_rs::Context;
use regex::Regex;
use tokio_postgres::Client;
use tracing::{info, instrument, trace};

use super::pool::{INIT_STATE, get_db};
use super::sql::{campaign, master};
use super::state::DatabaseState;
use crate::core::RUNTIME;
use crate::error::{QueryResult, QueryState, transient_error};

static CAMPAIGN_KEY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9_]{3,49}$").unwrap());

/// Sanitizes a key for use as a schema name. Errors if the key doesn't match
/// `^[a-z0-9_]{3,49}$` (after lowercasing and replacing `-`/space with `_`).
pub fn sanitize_key(key: &str) -> Result<String, &'static str> {
    let key = key.replace(['-', ' '], "_").to_lowercase();

    if !CAMPAIGN_KEY_RE.is_match(&key) {
        return Err("invalid key pattern");
    }

    Ok(key)
}

/// Translates the raw `campaign_id` Arma argument into the optional sanitized
/// key the bootstrap pipeline expects. Empty input means "master only";
/// non-empty input is validated through [`sanitize_key`].
pub(super) fn parse_campaign_arg(raw: &str) -> Result<Option<String>, &'static str> {
    if raw.is_empty() {
        Ok(None)
    } else {
        sanitize_key(raw).map(Some)
    }
}

/// Creates the master schema + tables + indexes and seeds the default rank.
/// Does NOT run the file-driven migrations — callers (production = bootstrap;
/// tests = direct) can compose this with whatever data step they need.
#[instrument(level = "debug", name = "bootstrap_schema", skip(client))]
pub(crate) async fn bootstrap_schema(client: &Client) -> Result<(), QueryResult> {
    // Order matters: each entry depends on (at most) those above it
    // (schema → base tables → tables with FKs → indexes).
    let statements: &[(&str, &str)] = &[
        (master::SCHEMA, "master schema"),
        (master::RANKS, "ranks table"),
        (master::CERTIFICATIONS, "certifications table"),
        (master::CAMPAIGNS, "campaigns table"),
        (master::PLAYER_INFO, "player_info table"),
        (master::PLAYER_INFO_IDX_ADMIN, "admin index"),
        (master::PLAYER_INFO_IDX_BANNED, "banned index"),
        (master::PLAYER_CERTS, "player_certs table"),
        (master::PLAYER_CERTS_IDX_STEAM, "player_certs steam index"),
        (master::PLAYER_CERTS_IDX_CERT, "player_certs cert index"),
        (master::MIGRATION_STATE, "migration_state table"),
    ];

    for (sql, desc) in statements {
        if let Err(e) = client.execute(*sql, &[]).await {
            return Err(transient_error(&format!("Failed to create {desc}"), e));
        }
    }

    // Seed the default rank (0, 'Unranked') so player_info.rank's FK default
    // is always satisfied. Files in database/migrations/ranks/ may override
    // the display_name via the migration UPSERT below.
    if let Err(e) = client
        .execute(
            "INSERT INTO skua_master.ranks (id, display_name) \
             VALUES (0, 'Unranked') \
             ON CONFLICT (id) DO NOTHING",
            &[],
        )
        .await
    {
        return Err(transient_error("Failed to seed default rank", e));
    }

    Ok(())
}

#[instrument(level = "debug", name = "bootstrap_master", skip(client))]
pub(super) async fn bootstrap_master(client: &Client) -> Result<(), QueryResult> {
    bootstrap_schema(client).await?;

    // File-driven reconciliation of cert/rank rows. Runs every bootstrap so
    // adds/removes from `database/migrations/` are picked up on server restart.
    crate::certification::migrate(client).await?;
    crate::ranks::migrate(client).await?;

    info!("master schema bootstrapped");
    Ok(())
}

#[instrument(level = "debug", name = "bootstrap_campaign", skip(client))]
pub(super) async fn bootstrap_campaign(
    client: &Client,
    campaign_id: &str,
) -> Result<(), QueryResult> {
    let statements: &[(&str, &str)] = &[
        (campaign::SCHEMA, "campaign schema"),
        (campaign::PLAYER_DATA, "player_data table"),
        (campaign::PLAYER_WORLD_DATA, "player_world_data table"),
        (campaign::WORLD_DATA, "world_data table"),
        (campaign::PLAYER_WORLD_DATA_IDX, "player_world_data index"),
        (campaign::WORLD_DATA_IDX, "world_data index"),
    ];

    for (template, desc) in statements {
        let sql = template.replace("${campaign_id}", campaign_id);
        if let Err(e) = client.execute(&sql, &[]).await {
            return Err(transient_error(&format!("Failed to create {desc}"), e));
        }
    }

    let register_sql = r"
        INSERT INTO skua_master.campaigns (campaign_id)
        VALUES ($1)
        ON CONFLICT (campaign_id) DO NOTHING
    ";
    let campaign_id_formatted = format!("skua_campaign_{campaign_id}");
    if let Err(e) = client
        .execute(register_sql, &[&campaign_id_formatted])
        .await
    {
        return Err(transient_error("Failed to register campaign", e));
    }

    info!(campaign_id = %campaign_id, "campaign schema bootstrapped");
    Ok(())
}

#[instrument(level = "debug", skip_all, fields(campaign_id = ?campaign_id))]
pub(super) async fn do_bootstrap(campaign_id: Option<String>) -> QueryResult {
    let db = match get_db().await {
        Ok(db) => db,
        Err(e) => {
            // Sticky terminal failure: OnceCell will retry on the next get_db()
            // call, but get_state() reports Failed in the meantime.
            INIT_STATE.store(DatabaseState::Failed);
            return transient_error("Failed to get database handle", e);
        }
    };

    let client = match db.get_conn().await {
        Ok(c) => c,
        Err(e) => return transient_error("Failed to get connection", e),
    };

    if let Err(result) = bootstrap_master(&client).await {
        return result;
    }

    if let Some(ref cid) = campaign_id
        && let Err(result) = bootstrap_campaign(&client, cid).await
    {
        return result;
    }

    db.set_state(DatabaseState::ConnectedInit);

    info!(campaign_id = ?campaign_id, "bootstrap complete");

    QueryResult::done()
}

#[allow(clippy::needless_pass_by_value)] // (Arma commands must take owned args.)
/// Arma-callable entry point. Spawns the bootstrap onto the global runtime and
/// returns `Processing`; result is delivered via `skua:database` callback.
pub fn bootstrap(ctx: Context, campaign_id: String) -> QueryState {
    trace!(%campaign_id, "bootstrap command entered");
    let Ok(campaign) = parse_campaign_arg(&campaign_id) else {
        trace!(%campaign_id, "bootstrap: invalid campaign_id arg, returning InvalidArgument");
        return QueryState::InvalidArgument;
    };

    trace!(?campaign, "spawning bootstrap task onto runtime");
    RUNTIME.spawn(async move {
        trace!(?campaign, "bootstrap task running on runtime");
        let result = do_bootstrap(campaign).await;
        trace!(
            ?result,
            "do_bootstrap returned; firing skua:database/bootstrap callback"
        );
        let success = result.state == QueryState::Done;
        let _ = ctx.callback_data("skua:database", "bootstrap", result);

        // Static-data push (cert list, rank list, ...) runs AFTER the bootstrap
        // callback so the SQF side learns the DB is up even if a push panics.
        // Each domain has its own `loaded` event — `database/initialized` does
        // NOT imply cert/rank lists have landed in SQF yet.
        if success {
            crate::sync::trigger_post_bootstrap();
        }
    });

    QueryState::Processing
}
