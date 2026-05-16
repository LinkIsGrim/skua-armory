//! Global Tokio runtime for async operations.
//!
//! All async work should be spawned onto this runtime.

use std::sync::LazyLock;

pub static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to initialize tokio runtime")
});
