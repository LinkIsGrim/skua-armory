//! Post-bootstrap static-data push orchestration + live-load watchdog.
//!
//! Two responsibilities:
//! - [`trigger_post_bootstrap`] fires the one-shot push of "relatively static"
//!   server data (cert list, rank list, ...) after `bootstrap_master` succeeds.
//!   SQF subscribes to the resulting callbacks; no SQF-initiated request needed.
//! - [`watchdog`] runs a periodic per-player cert diff against the DB so SQF
//!   picks up DB-side cert changes without a reconnect.
//!
//! The post-bootstrap push runs as a fire-and-forget background task: the
//! bootstrap pipeline triggers it and returns immediately. Each domain's own
//! `loaded` event is the canonical "data ready" signal for consumers.
//!
//! New static datasets join by adding one line in [`push_all`].

pub(crate) mod watchdog;

#[cfg(test)]
mod tests;

use tracing::{error, instrument};

use crate::core::{CONTEXT, RUNTIME};
use crate::database::get_client;

/// Kicks off the post-bootstrap push on the global runtime and returns
/// immediately. Safe to call from within another async task — the spawned
/// future is detached.
///
/// Also starts the live-load watchdog (idempotent — see
/// [`watchdog::spawn`]). Bootstrap only ever runs on the machine that owns
/// the DB connection (SQF gates `database:bootstrap` behind `isServer`), so
/// this is the right place to start DB-polling background work — starting it
/// unconditionally at extension load would have every client polling a DB it
/// has no business talking to.
pub(crate) fn trigger_post_bootstrap() {
    watchdog::spawn();
    RUNTIME.spawn(push_all());
}

#[instrument(level = "debug", name = "sync_push_post_bootstrap", skip_all)]
async fn push_all() {
    let Some(ctx) = CONTEXT.get() else {
        error!("post-bootstrap push aborted: extension context not initialized");
        return;
    };
    let client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            error!(error = %e, "post-bootstrap push aborted: failed to acquire client");
            return;
        }
    };
    crate::certification::push_list(ctx, &client).await;
    crate::ranks::push_list(ctx, &client).await;
}
