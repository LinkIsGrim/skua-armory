//! Arma-callable player lifecycle commands.
//!
//! `player_connect` is the single SQF-facing entry point fired when a player
//! joins. It (1) records the player in [`crate::core::CONNECTED`] so live-load
//! flows can target online players, (2) upserts the row in
//! `skua_master.player_info` (insert seeds `name`; conflict updates `name` and
//! bumps `last_seen`), and (3) replays the player's stored cert grants via
//! [`crate::certification::push_player_certs`].
//!
//! On success, emits [`crate::event::Event::PlayerConnected`] with the upserted
//! `PlayerInfo`. This is a critical event ([`Event::is_critical`]) — `emit`
//! retries the SQF callback on failure. The SQF side additionally runs a
//! per-player timeout-retry on the original `callExtension` call to cover
//! strand modes the Rust retry can't see (SQF handler crash, mission still
//! parsing). The upsert is idempotent, so retries are no-op DB writes.
//!
//! Cert hydration via [`crate::certification::push_player_certs`] emits its own
//! [`crate::event::Event::CertificationGranted`] events independently — no
//! ordering guarantee against `PlayerConnected`.
//!
//! `player_disconnect` emits [`crate::event::Event::PlayerDisconnected`] before
//! removing the player from `CONNECTED`. `last_seen` is bumped on the next
//! connect's upsert.

use arma_rs::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio_postgres::Client;
use tracing::{debug, error, instrument};

use crate::core::{CONNECTED, RUNTIME};
use crate::database::get_client;
use crate::domain::PlayerId;
use crate::error::{QueryError, QueryState, transient_query_error};
use crate::event::{self, Event};

/// JSON payload mirroring `skua_master.player_info`. Timestamps serialize as
/// RFC3339 strings (chrono's default Serialize impl).
///
/// `Clone` is required so the event bus can carry `PlayerInfo` to internal
/// Rust subscribers; `pub` so the event module can name it in
/// [`crate::event::Event::PlayerConnected`] (which is itself `pub`).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PlayerInfo {
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
                error!(error = ?e, %player_id, "player_connect: failed to get database client");
                return;
            }
        };

        let info = match upsert_player_inner(&client, player_id, &name).await {
            Ok(info) => info,
            Err(err) => {
                error!(%err, %player_id, "player_connect: upsert failed");
                return;
            }
        };

        event::emit(&ctx, &Event::PlayerConnected { info });

        // Cert hydration replays grants the player already owns via
        // Event::CertificationGranted. No ordering guarantee against
        // PlayerConnected — both flow on the same event channel and SQF
        // handlers must tolerate either order (see fnc_onCertificationGranted's
        // pendingCertEvents queue).
        let cert_ids = crate::certification::push_player_certs(&ctx, &client, player_id).await;
        // Seed the watchdog snapshot so the next periodic tick has the correct
        // baseline; without this, the first tick would re-fire grants for
        // every cert the player already received above.
        crate::sync::watchdog::seed_player(player_id, cert_ids);
    });

    QueryState::Processing
}

#[allow(clippy::needless_pass_by_value)] // (Arma commands must take owned args.)
pub fn player_disconnect(ctx: Context, player_id: PlayerId, name: String) -> QueryState {
    let removed = CONNECTED
        .write()
        .expect("CONNECTED lock poisoned")
        .remove(&player_id);
    crate::sync::watchdog::clear_player(player_id);
    event::emit(&ctx, &Event::PlayerDisconnected { player_id });
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
