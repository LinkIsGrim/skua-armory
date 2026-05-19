//! Unified event surface for Rust↔SQF state-change broadcasts.
//!
//! Replaces the per-group callback channels (`skua:certification`, `skua:ranks`,
//! `skua:database`) with one [`APP_CHANNEL`] (`skua:event`). The `_function`
//! slot of the arma-rs callback carries the event name verbatim (e.g.
//! `"skua_certification_granted"`); the SQF adapter at
//! `addons/common/functions/fnc_extCallback_event.sqf` re-emits it onto CBA via
//! `CBA_fnc_localEvent` so consumers subscribe with `CBA_fnc_addEventHandler`.
//!
//! Events are **broadcast-only**: they never carry "your request finished"
//! semantics. Ack-needing operations use sync commands via [`sync::run_sync`]
//! and return their outcome directly from `callExtension`.
//!
//! `emit` also publishes to an internal [`broadcast`] bus so Rust-side modules
//! (future audit logging, stats aggregation, achievement tracking, batched
//! telemetry export) can subscribe via [`subscribe`] without needing any
//! plumbing in the emit sites.
//!
//! Event-name strings MUST stay in sync with `addons/main/script_macros_events.hpp`.
//! The check in `crate::enum_sync` enforces this at `cargo test` time and also
//! verifies every event has at least one SQF subscriber.

use std::sync::LazyLock;
use std::time::Duration;

use arma_rs::Context;
use serde::Serialize;
use tokio::sync::broadcast;
use tracing::error;

use crate::certification::Certification;
use crate::core::{CONTEXT, RUNTIME};
use crate::database::PlayerInfo;
use crate::domain::PlayerId;
use crate::error::QueryOutcome;
use crate::ranks::Rank;

pub mod sync;

#[cfg(test)]
mod tests;

/// Wire-side channel name for app-class events. Infra channels (logging,
/// database state) bypass this and use their own callback names.
pub const APP_CHANNEL: &str = "skua:event";

/// Capacity of the internal broadcast bus. Subscribers lagging by more than
/// this many events receive [`broadcast::error::RecvError::Lagged`] and decide
/// whether to skip ahead or escalate. Sized generously since events are
/// human-scale frequency.
const BUS_CAPACITY: usize = 1024;

/// Number of additional callback attempts after the synchronous first attempt
/// fails for a critical event. Total attempts = 1 + this value.
const CRITICAL_RETRY_ATTEMPTS: usize = 2;

/// Backoff between critical-event retry attempts. Short because callback
/// failures usually mean the engine momentarily refused; not a slow recovery.
const CRITICAL_RETRY_BACKOFF: Duration = Duration::from_millis(5);

static BUS: LazyLock<broadcast::Sender<Event>> = LazyLock::new(|| {
    let (tx, _) = broadcast::channel(BUS_CAPACITY);
    tx
});

/// Subscribe a Rust-side listener to the event bus. Subscribers see events
/// emitted after they subscribe; no replay. Spawn a task on
/// [`crate::core::RUNTIME`] and loop on `rx.recv().await`, matching on the
/// variants the subscriber cares about.
pub fn subscribe() -> broadcast::Receiver<Event> {
    BUS.subscribe()
}

#[derive(Debug, Clone)]
pub enum Event {
    CertificationGranted {
        player_id: PlayerId,
        cert_id: String,
    },
    CertificationRevoked {
        player_id: PlayerId,
        cert_id: String,
    },
    CertificationListChanged {
        certs: Vec<Certification>,
    },
    RankChanged {
        player_id: PlayerId,
        rank_id: i16,
    },
    RankListChanged {
        ranks: Vec<Rank>,
    },
    /// Player record was upserted on join. Does NOT imply cert hydration is
    /// complete — subscribe to [`Event::CertificationGranted`] for that.
    /// Critical: the SQF side has no convergence path if this is dropped, so
    /// [`Event::is_critical`] returns `true` and `emit` retries on failure.
    PlayerConnected {
        info: PlayerInfo,
    },
    PlayerDisconnected {
        player_id: PlayerId,
    },
}

impl Event {
    /// Stable wire name. MUST match the corresponding `QEV_*` macro in
    /// `addons/main/script_macros_events.hpp`. The `enum_sync` test enforces this.
    #[must_use]
    pub fn event_name(&self) -> &'static str {
        match self {
            Event::CertificationGranted { .. } => "skua_certification_granted",
            Event::CertificationRevoked { .. } => "skua_certification_revoked",
            Event::CertificationListChanged { .. } => "skua_certification_list_changed",
            Event::RankChanged { .. } => "skua_rank_changed",
            Event::RankListChanged { .. } => "skua_rank_list_changed",
            Event::PlayerConnected { .. } => "skua_player_connected",
            Event::PlayerDisconnected { .. } => "skua_player_disconnected",
        }
    }

    /// Critical events get a Rust-side retry path if the initial callback
    /// fails. A dropped callback for a critical event would strand state on
    /// the SQF side with no convergence; see also the SQF-side timeout-retry
    /// in `addons/persistence` for `PlayerConnected`.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        matches!(self, Event::PlayerConnected { .. })
    }

    /// Serialize the variant data as JSON. Field naming MUST match what SQF
    /// handlers read via `fromJSON`. Variants carrying a list (`*ListChanged`)
    /// serialize the list directly so SQF gets a top-level JSON array.
    /// # Errors
    /// If serialization fails. SQF handlers MUST be robust to this by treating
    /// payloads as opaque strings and logging/ignoring on parse failure, so this is not expected to be a common occurrence.
    pub fn payload(&self) -> Result<String, serde_json::Error> {
        match self {
            Event::CertificationGranted { player_id, cert_id }
            | Event::CertificationRevoked { player_id, cert_id } => {
                #[derive(Serialize)]
                struct P<'a> {
                    player_id: PlayerId,
                    cert_id: &'a str,
                }
                serde_json::to_string(&P {
                    player_id: *player_id,
                    cert_id,
                })
            }
            Event::CertificationListChanged { certs } => serde_json::to_string(certs),
            Event::RankChanged { player_id, rank_id } => {
                #[derive(Serialize)]
                struct P {
                    player_id: PlayerId,
                    rank_id: i16,
                }
                serde_json::to_string(&P {
                    player_id: *player_id,
                    rank_id: *rank_id,
                })
            }
            Event::RankListChanged { ranks } => serde_json::to_string(ranks),
            Event::PlayerConnected { info } => serde_json::to_string(info),
            Event::PlayerDisconnected { player_id } => {
                #[derive(Serialize)]
                struct P {
                    player_id: PlayerId,
                }
                serde_json::to_string(&P {
                    player_id: *player_id,
                })
            }
        }
    }
}

/// Publish an event to internal Rust subscribers and to SQF on [`APP_CHANNEL`].
///
/// Non-critical events log on callback failure and continue. Critical events
/// (currently just [`Event::PlayerConnected`]) additionally spawn a background
/// retry task on [`RUNTIME`] using [`crate::core::CONTEXT`].
pub fn emit(ctx: &Context, event: &Event) {
    // 1. Internal broadcast first. `Err` only means zero subscribers — drop
    //    silently. Subscribers see events even if SQF callback fails below.
    let _ = BUS.send(event.clone());

    // 2. SQF callback.
    let name = event.event_name();
    let payload = match event.payload() {
        Ok(p) => p,
        Err(e) => {
            error!(error = ?e, event_name = name, "failed to serialize event payload");
            return;
        }
    };

    let outcome: QueryOutcome<String> = QueryOutcome::Done(payload.clone());
    let first_result = ctx.callback_data(APP_CHANNEL, name, outcome);

    match first_result {
        Ok(()) => {}
        Err(e) if !event.is_critical() => {
            error!(error = ?e, event_name = name, "event callback failed");
        }
        Err(e) => {
            error!(error = ?e, event_name = name, "critical event callback failed; spawning retry");
            spawn_critical_retry(name, payload);
        }
    }
}

fn spawn_critical_retry(name: &'static str, payload: String) {
    RUNTIME.spawn(async move {
        let Some(ctx) = CONTEXT.get() else {
            error!(
                event_name = name,
                "CONTEXT not initialized; cannot retry critical event"
            );
            return;
        };
        for attempt in 1..=CRITICAL_RETRY_ATTEMPTS {
            tokio::time::sleep(CRITICAL_RETRY_BACKOFF).await;
            let outcome: QueryOutcome<String> = QueryOutcome::Done(payload.clone());
            if ctx.callback_data(APP_CHANNEL, name, outcome).is_ok() {
                return;
            }
            if attempt == CRITICAL_RETRY_ATTEMPTS {
                error!(
                    event_name = name,
                    attempts = CRITICAL_RETRY_ATTEMPTS + 1,
                    "critical event emit failed after all attempts"
                );
            }
        }
    });
}
