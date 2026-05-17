//! Arma-callable commands for log level management.

use arma_rs::Group;
use tracing::Level;

use super::LOG_LEVEL;

/// Set the log level dynamically. Valid levels: ERROR, WARN, INFO, DEBUG, TRACE.
///
/// Returns the new level on success, or an error message describing why the
/// request was rejected (unknown level, poisoned lock).
pub fn set_level(level: String) -> Result<String, String> {
    let new_level = match level.to_uppercase().as_str() {
        "ERROR" => Level::ERROR,
        "WARN" => Level::WARN,
        "INFO" => Level::INFO,
        "DEBUG" => Level::DEBUG,
        "TRACE" => Level::TRACE,
        other => return Err(format!("unknown log level: {other}")),
    };

    match LOG_LEVEL.write() {
        Ok(mut current) => {
            *current = new_level;
            Ok(get_level())
        }
        Err(_) => Err("LOG_LEVEL lock poisoned".into()),
    }
}

pub fn get_level() -> String {
    LOG_LEVEL
        .read().map_or_else(|_| "INFO".to_string(), |level| level.to_string())
}

pub fn group() -> Group {
    Group::new()
        .command("set_level", set_level)
        .command("get_level", get_level)
}
