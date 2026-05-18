//! Global extension context.
//!
//! Holds the `arma_rs::Context` provided by the extension init function so
//! background tasks (post-bootstrap pushes, future periodic syncs) can fire
//! callbacks without threading the context through every call site.

use std::sync::OnceLock;

use arma_rs::Context;

pub static CONTEXT: OnceLock<Context> = OnceLock::new();
