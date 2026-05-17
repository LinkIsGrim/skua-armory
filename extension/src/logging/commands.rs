//! Arma-callable commands for log level management.

use arma_rs::Group;
use tracing::{Level, debug, error, info, trace, warn};

use super::LOG_LEVEL;

#[allow(clippy::needless_pass_by_value)] // (Arma commands must take owned args.)
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

    info!("Setting log level to {new_level}");

    match LOG_LEVEL.write() {
        Ok(mut current) => {
            *current = new_level;
            drop(current);
        }
        Err(_) => return Err("LOG_LEVEL lock poisoned".into()),
    }

    match new_level {
        Level::ERROR => error!("log filter test message"),
        Level::WARN => warn!("log filter test message"),
        Level::INFO => info!("log filter test message"),
        Level::DEBUG => debug!("log filter test message"),
        Level::TRACE => trace!("log filter test message"),
    }

    Ok(get_level())
}

pub fn get_level() -> String {
    LOG_LEVEL
        .read()
        .map_or_else(|_| "INFO".to_string(), |level| level.to_string())
}

pub fn group() -> Group {
    Group::new()
        .command("set_level", set_level)
        .command("get_level", get_level)
}
