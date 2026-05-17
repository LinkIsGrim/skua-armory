//! Centralized error handling for the extension.
//!
//! Unified error types and helpers for converting errors into Arma-compatible
//! responses.

mod db;
mod query;

pub use db::DbError;
pub use query::{QueryError, QueryOutcome, QueryResult, QueryState};

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

/// Same shape as [`transient_error`] but returns the bare [`QueryError`], for
/// use with [`QueryOutcome::Failed`] payloads where the caller needs the error
/// detail without the surrounding state field.
#[track_caller]
pub fn transient_query_error<E>(message: &str, error: E) -> QueryError
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

    QueryError {
        code: "UNAVAILABLE".to_string(),
        message: format!("{}: {}", message, error),
        location: format!("{}:{}", loc.file(), loc.line()),
    }
}
