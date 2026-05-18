//! Arma-callable player lifecycle commands.
//!
//! `player_connect` is the single SQF-facing entry point fired when a player
//! joins. It (1) records the player in [`crate::core::CONNECTED`] so live-load
//! flows can target online players, (2) upserts the row in
//! `skua_master.player_info` (insert seeds `name`; conflict updates `name` and
//! bumps `last_seen`), and (3) replays the player's stored cert grants via
//! [`crate::certification::push_player_certs`].
//!
//! The upsert result is delivered on `skua:database/player_connect` so SQF can
//! attach the row to the player unit and fire `skua_database_playerReady`. The
//! cert grants land on `skua:certification/grant`, one callback per cert.
//!
//! `player_disconnect` is fire-and-forget: it just removes the player from
//! `CONNECTED`. `last_seen` is bumped on the next connect's upsert.

use arma_rs::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio_postgres::Client;
use tracing::{debug, error, instrument};

use crate::core::{CONNECTED, RUNTIME};
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
pub fn player_connect(ctx: Context, player_id: PlayerId, name: String) -> QueryState {
    CONNECTED
        .write()
        .expect("CONNECTED lock poisoned")
        .insert(player_id);
    debug!(player_id = %player_id, name = %name, "player connected");

    RUNTIME.spawn(async move {
        let client = match get_client().await {
            Ok(c) => c,
            Err(e) => {
                let outcome: QueryOutcome<String> =
                    QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
                if let Err(e) = ctx.callback_data("skua:database", "player_connect", outcome) {
                    error!(error = ?e, "failed to dispatch database:player_connect callback");
                }
                return;
            }
        };

        let upsert_outcome: QueryOutcome<String> =
            match upsert_player_inner(&client, player_id, &name).await {
                Ok(info) => match serde_json::to_string(&info) {
                    Ok(json) => QueryOutcome::Done(json),
                    Err(e) => QueryOutcome::Failed(transient_query_error(
                        "Failed to serialize player_info",
                        e,
                    )),
                },
                Err(err) => QueryOutcome::Failed(err),
            };

        let upsert_ok = matches!(upsert_outcome, QueryOutcome::Done(_));
        if let Err(e) = ctx.callback_data("skua:database", "player_connect", upsert_outcome) {
            error!(error = ?e, "failed to dispatch database:player_connect callback");
        }

        // Skip cert replay if the upsert failed — SQF will retry the whole
        // connect when the transient failure clears.
        if upsert_ok {
            let cert_ids = crate::certification::push_player_certs(&ctx, &client, player_id).await;
            // Seed the watchdog snapshot so the next periodic tick has the
            // correct baseline; without this, the first tick would re-fire
            // grants for every cert the player already received above.
            crate::sync::watchdog::seed_player(player_id, cert_ids);
        }
    });

    QueryState::Processing
}

#[allow(clippy::needless_pass_by_value)] // (Arma commands must take owned args.)
pub fn player_disconnect(player_id: PlayerId, name: String) -> QueryState {
    let removed = CONNECTED
        .write()
        .expect("CONNECTED lock poisoned")
        .remove(&player_id);
    crate::sync::watchdog::clear_player(player_id);
    debug!(player_id = %player_id, name = %name, removed, "player disconnected");
    QueryState::Done
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
