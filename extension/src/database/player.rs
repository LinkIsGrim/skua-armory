//! Arma-callable `player_info` commands.
//!
//! `upsert_player` upserts a row in `skua_master.player_info` keyed on
//! `steam_id`. Insert seeds `name`; conflict updates `name` and bumps
//! `last_seen` to `NOW()`. Returns the full row to SQF so the server can
//! attach it to the player unit and fire `skua_database_playerReady`.

use arma_rs::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio_postgres::Client;
use tracing::{error, instrument};

use crate::core::RUNTIME;
use crate::database::get_client;
use crate::domain::PlayerId;
use crate::error::{QueryError, QueryOutcome, QueryState, transient_query_error};

/// JSON payload mirroring `skua_master.player_info`. Timestamps serialize as
/// RFC3339 strings (chrono's default Serialize impl).
#[derive(Debug, Serialize, PartialEq, Eq)]
pub(super) struct PlayerInfo {
    pub steam_id: PlayerId,
    pub name: String,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub is_admin: bool,
    pub is_banned: bool,
    pub rank: i16,
}

#[allow(clippy::needless_pass_by_value)] // (Arma commands must take owned args.)
pub fn upsert_player(ctx: Context, player_id: PlayerId, name: String) -> QueryState {
    RUNTIME.spawn(async move {
        let outcome = run_upsert_player(player_id, name).await;
        if let Err(e) = ctx.callback_data("skua:database", "upsert_player", outcome) {
            error!(error = ?e, "failed to dispatch database:upsert_player callback");
        }
    });
    QueryState::Processing
}

async fn run_upsert_player(player_id: PlayerId, name: String) -> QueryOutcome<String> {
    let client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            return QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
        }
    };
    match upsert_player_inner(&client, player_id, &name).await {
        Ok(info) => match serde_json::to_string(&info) {
            Ok(json) => QueryOutcome::Done(json),
            Err(e) => {
                QueryOutcome::Failed(transient_query_error("Failed to serialize player_info", e))
            }
        },
        Err(err) => QueryOutcome::Failed(err),
    }
}

#[instrument(level = "debug", name = "database_upsert_player", skip(client), fields(player_id = %player_id))]
pub(super) async fn upsert_player_inner(
    client: &Client,
    player_id: PlayerId,
    name: &str,
) -> Result<PlayerInfo, QueryError> {
    let row = client
        .query_one(
            "INSERT INTO skua_master.player_info (steam_id, name)
             VALUES ($1, $2)
             ON CONFLICT (steam_id) DO UPDATE
                 SET name = EXCLUDED.name,
                     last_seen = NOW()
             RETURNING steam_id, name, first_seen, last_seen, is_admin, is_banned, rank",
            &[&player_id, &name],
        )
        .await
        .map_err(|e| transient_query_error("Failed to upsert player_info", e))?;

    Ok(PlayerInfo {
        steam_id: PlayerId::from_i64(row.get::<_, i64>("steam_id")),
        name: row.get("name"),
        first_seen: row.get("first_seen"),
        last_seen: row.get("last_seen"),
        is_admin: row.get("is_admin"),
        is_banned: row.get("is_banned"),
        rank: row.get("rank"),
    })
}
