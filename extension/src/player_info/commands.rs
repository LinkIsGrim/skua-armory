//! Arma-callable `player_info` commands.
//!
//! `list` fires a callback on `skua:player_info/list` with a [`QueryOutcome`]:
//! - `[Done, payload]` on success — payload is a JSON string (see
//!   [`RosterEntry`] for the row shape) so SQF consumes via
//!   `fromJSON _payload` and gets `Vec<HashMap>` with `"steam_id"` / `"name"`
//!   keys directly. Server-sorted by `last_seen DESC` so the freshest players
//!   appear first in admin pickers without the SQF side needing the timestamp.
//! - `[TransientFailure, error]` on failure.
//!
//! Rows with empty `name` are skipped — those are stub upserts seeded by
//! `certification::grant_inner`'s FK fallback for granters that have never
//! connected and shouldn't appear in admin pickers.

use arma_rs::{Context, Group};
use tokio_postgres::Client;
use tracing::{error, instrument};

use crate::core::RUNTIME;
use crate::database::get_client;
use crate::domain::PlayerId;
use crate::error::{QueryError, QueryOutcome, QueryState, transient_query_error};
use crate::player_info::types::{Roster, RosterEntry};

pub fn group() -> Group {
    Group::new().command("list", list)
}

fn list(ctx: Context) -> QueryState {
    RUNTIME.spawn(async move {
        let outcome = run_list().await;
        if let Err(e) = ctx.callback_data("skua:player_info", "list", outcome) {
            error!(error = ?e, "failed to dispatch player_info:list callback");
        }
    });
    QueryState::Processing
}

async fn run_list() -> QueryOutcome<Roster> {
    let client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            return QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
        }
    };
    match list_inner(&client).await {
        Ok(roster) => QueryOutcome::Done(roster),
        Err(err) => QueryOutcome::Failed(err),
    }
}

#[instrument(level = "debug", name = "player_info_list", skip_all)]
pub(crate) async fn list_inner(client: &Client) -> Result<Roster, QueryError> {
    let rows = client
        .query(
            "SELECT steam_id, name
             FROM skua_master.player_info
             WHERE name <> ''
             ORDER BY last_seen DESC",
            &[],
        )
        .await
        .map_err(|e| transient_query_error("Failed to query player_info", e))?;

    Ok(Roster(
        rows.iter()
            .map(|row| RosterEntry {
                steam_id: PlayerId::from_i64(row.get::<_, i64>("steam_id")),
                name: row.get("name"),
            })
            .collect(),
    ))
}
