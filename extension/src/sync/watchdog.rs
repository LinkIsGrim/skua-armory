//! Periodic cert-state watchdog.
//!
//! Tracks a `PlayerId → {cert_id, ...}` snapshot of what each online player
//! should have in-game. Every `SKUA_WATCHDOG_INTERVAL_SECS` (default 300) the
//! tick queries the DB for each connected player's current cert set, diffs
//! against the snapshot, and fires `skua:certification/grant` /
//! `skua:certification/revoke` callbacks for the differences so SQF picks up
//! DB-side changes without re-connect.
//!
//! Seeding:
//! - `database:player_connect` calls [`seed_player`] after `push_player_certs`
//!   completes, so the watchdog's first observation of the player has the
//!   correct baseline (no double-grant on first tick).
//! - `database:player_disconnect` calls [`clear_player`].
//!
//! Players in `CONNECTED` with no entry in [`WATCHDOG_STATE`] are skipped this
//! tick — that means `player_connect` is still resolving and seeding hasn't
//! happened yet.

use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::{LazyLock, RwLock};
use std::time::Duration;

use arma_rs::Context;
use tokio_postgres::Client;
use tracing::{debug, error, info, instrument, warn};

use crate::certification::{dispatch_grant_event, dispatch_revoke_event, get_player_inner};
use crate::core::{CONNECTED, CONTEXT, RUNTIME};
use crate::database::get_client;
use crate::domain::PlayerId;

const DEFAULT_TICK_SECS: u64 = 30;
const TICK_ENV_VAR: &str = "SKUA_WATCHDOG_INTERVAL_SECS";

static WATCHDOG_STATE: LazyLock<RwLock<HashMap<PlayerId, HashSet<String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Records the player's current cert set. Called from `database:player_connect`
/// after `push_player_certs` has emitted the initial grant callbacks; the
/// snapshot serves as the baseline for the next watchdog tick.
pub(crate) fn seed_player(player_id: PlayerId, cert_ids: Vec<String>) {
    let set: HashSet<String> = cert_ids.into_iter().collect();
    WATCHDOG_STATE
        .write()
        .expect("WATCHDOG_STATE poisoned")
        .insert(player_id, set);
}

/// Drops the player's snapshot. Called from `database:player_disconnect`.
pub(crate) fn clear_player(player_id: PlayerId) {
    WATCHDOG_STATE
        .write()
        .expect("WATCHDOG_STATE poisoned")
        .remove(&player_id);
}

/// Spawns the periodic watchdog task on the global runtime. Idempotent in
/// practice — only `lib.rs::init` calls it, and only once per extension load.
pub(crate) fn spawn() {
    let interval = tick_interval();
    info!(interval_secs = interval.as_secs(), "starting cert watchdog");
    RUNTIME.spawn(async move {
        let mut ticker = tokio::time::interval(interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        // Burn the immediate first tick — there is nothing to diff before any
        // player has connected.
        ticker.tick().await;
        loop {
            ticker.tick().await;
            tick().await;
        }
    });
}

fn tick_interval() -> Duration {
    let secs = env::var(TICK_ENV_VAR)
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|s| *s > 0)
        .unwrap_or(DEFAULT_TICK_SECS);
    Duration::from_secs(secs)
}

#[instrument(level = "debug", name = "watchdog_tick", skip_all)]
async fn tick() {
    let players: Vec<PlayerId> = CONNECTED
        .read()
        .expect("CONNECTED poisoned")
        .iter()
        .copied()
        .collect();
    if players.is_empty() {
        return;
    }

    let Some(ctx) = CONTEXT.get() else {
        warn!("watchdog tick aborted: extension context not initialized");
        return;
    };

    let client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            warn!(error = %e, "watchdog tick: failed to acquire client; will retry next tick");
            return;
        }
    };

    debug!(player_count = players.len(), "watchdog tick");
    for player_id in players {
        diff_player(ctx, &client, player_id).await;
    }
}

async fn diff_player(ctx: &Context, client: &Client, player_id: PlayerId) {
    // Snapshot the seeded set. If the player has no entry, player_connect
    // hasn't finished seeding yet — skip and let next tick pick them up.
    let old_set = {
        let map = WATCHDOG_STATE.read().expect("WATCHDOG_STATE poisoned");
        match map.get(&player_id) {
            Some(set) => set.clone(),
            None => return,
        }
    };

    let new_set: HashSet<String> = match get_player_inner(client, player_id).await {
        Ok(ids) => ids.into_iter().collect(),
        Err(err) => {
            error!(player_id = %player_id, error = %err, "watchdog: failed to query certs");
            return;
        }
    };

    for cert_id in new_set.difference(&old_set) {
        debug!(player_id = %player_id, %cert_id, "watchdog detected new grant");
        dispatch_grant_event(ctx, player_id, cert_id);
    }
    for cert_id in old_set.difference(&new_set) {
        debug!(player_id = %player_id, %cert_id, "watchdog detected revoke");
        dispatch_revoke_event(ctx, player_id, cert_id);
    }

    // Disconnect-during-tick guard: don't re-seed the map for a player who
    // is no longer connected. clear_player would have removed them; we
    // mustn't resurrect the entry.
    let still_connected = CONNECTED
        .read()
        .expect("CONNECTED poisoned")
        .contains(&player_id);
    if still_connected {
        WATCHDOG_STATE
            .write()
            .expect("WATCHDOG_STATE poisoned")
            .insert(player_id, new_set);
    }
}
