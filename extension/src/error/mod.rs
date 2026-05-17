//! Centralized error handling for the extension.
//!
//! Unified error types and helpers for converting errors into Arma-compatible
//! responses.

mod db;
mod query;

pub use db::DbError;
pub use query::{QueryError, QueryResult, QueryState};

use std::error::Error;
use tracing::error;

/// Logs the error with caller location and returns a transient-failure
/// `QueryResult`. Use for recoverable errors that should be reported back to
/// Arma.
#[track_caller]
pub fn transient_error<E>(message: &str, error: E) -> QueryResult
where
    E: Error,
{
    let loc = std::panic::Location::caller();

    error!(
        error = %error,
        file = loc.file(),
        line = loc.line(),
        column = loc.column(),
        "{}",
        message
    );

    QueryResult {
        state: QueryState::TransientFailure,
        error: Some(QueryError {
            code: "UNAVAILABLE".to_string(),
            message: format!("{}: {}", message, error),
            location: format!("{}:{}", loc.file(), loc.line()),
        }),
    }
}
