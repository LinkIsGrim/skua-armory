//! Structured logging infrastructure.
//!
//! Tracing layer that forwards log events to Arma via extension callbacks.

mod commands;
mod layer;

pub use commands::group;
pub use layer::init;

use std::sync::RwLock;
use tracing::Level;

/// Global log level filter (default: INFO). RwLock allows dynamic changes via
/// `logger:set_level`.
pub(crate) static LOG_LEVEL: RwLock<Level> = RwLock::new(Level::INFO);
