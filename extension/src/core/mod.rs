//! Core infrastructure shared across all modules.

mod connected;
mod context;
mod runtime;

pub use connected::CONNECTED;
pub use context::CONTEXT;
pub use runtime::RUNTIME;
