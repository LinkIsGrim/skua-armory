//! Synchronous ack-bearing command helper.
//!
//! For operations where the SQF caller must receive an immediate outcome
//! before continuing (purchases, claims of limited-quantity items, ID
//! generation), an arma-rs command can call [`run_sync`] to block on a fast
//! database future and return [`SyncOutcome`] directly from `callExtension`.
//!
//! ## Latency budget
//!
//! Wraps the future in `tokio::time::timeout` with [`SYNC_TIMEOUT`]. Because
//! `callExtension` blocks the SQF scheduler for the entire call, the timeout
//! is deliberately tight: there is a hard 20ms ceiling on what we can afford
//! to block for in the worst case, so the default budget is reserved for
//! warm-pool localhost INSERTs and similar. Anything that might exceed the
//! budget (cold pool acquire, multi-row queries, contention-sensitive paths)
//! MUST stay async and emit events via [`super::emit`] instead.
//!
//! ## Re-entrancy
//!
//! [`run_sync`] uses `RUNTIME.block_on(...)` and MUST NOT be called from
//! inside an existing Tokio context — doing so panics with "Cannot block the
//! current thread from within a runtime". arma-rs command handlers run on a
//! worker thread that is NOT on the runtime, so the top-level entry path is
//! safe. Calls from inside futures spawned via [`crate::core::RUNTIME`] are
//! not. A `debug_assert!` guards this in debug builds.

use std::future::Future;
use std::time::Duration;

use arma_rs::IntoArma;

use crate::core::RUNTIME;
use crate::error::{QueryError, QueryState};

/// Worst-case time `run_sync` will block the SQF scheduler. See module docs.
pub const SYNC_TIMEOUT: Duration = Duration::from_millis(50);

/// Outcome of a synchronous command. Wire format matches
/// [`crate::error::QueryOutcome`] plus a [`QueryState::Timeout`] case.
#[derive(Debug)]
pub enum SyncOutcome<T> {
    Ok(T),
    Failed(QueryError),
    Timeout,
}

impl<T: IntoArma> IntoArma for SyncOutcome<T> {
    fn to_arma(&self) -> arma_rs::Value {
        match self {
            SyncOutcome::Ok(data) => {
                arma_rs::Value::Array(vec![QueryState::Done.to_arma(), data.to_arma()])
            }
            SyncOutcome::Failed(err) => {
                arma_rs::Value::Array(vec![QueryState::TransientFailure.to_arma(), err.to_arma()])
            }
            SyncOutcome::Timeout => arma_rs::Value::Array(vec![QueryState::Timeout.to_arma()]),
        }
    }
}

/// Block the calling thread on `fut` for at most [`SYNC_TIMEOUT`] and return
/// the outcome. See module docs for re-entrancy constraints.
pub fn run_sync<T, F>(fut: F) -> SyncOutcome<T>
where
    F: Future<Output = Result<T, QueryError>>,
{
    debug_assert!(
        tokio::runtime::Handle::try_current().is_err(),
        "run_sync called from inside a Tokio context; would panic. \
         arma-rs handlers must call this from outside any RUNTIME.spawn task."
    );

    match RUNTIME.block_on(tokio::time::timeout(SYNC_TIMEOUT, fut)) {
        Ok(Ok(value)) => SyncOutcome::Ok(value),
        Ok(Err(err)) => SyncOutcome::Failed(err),
        Err(_elapsed) => SyncOutcome::Timeout,
    }
}
