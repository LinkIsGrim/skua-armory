//! Skua extension entry point.

use std::collections::HashMap;

use arma_rs::{Extension, arma};
use uuid::Uuid;

pub mod core;
pub use core::RUNTIME;

pub mod error;
pub use error::{DbError, QueryError, QueryResult, QueryState};

pub mod database;
pub mod editor;
pub mod logging;

#[arma]
fn init() -> Extension {
    let ext = Extension::build()
        // Top-level commands
        .command("descriptionExt", editor::description_ext) // "skua" callExtension ["descriptionExt", [getMissionPath ""]]
        .command("uuid", Uuid::now_v7) // "skua" callExtension ["uuid", []]
        .command("diagnostics", diagnostics)
        // Command groups
        .group("logger", logging::group())
        .group("database", database::group())
        .finish();

    logging::init(ext.context());

    ext
}

fn diagnostics() -> HashMap<&'static str, String> {
    let mut output: HashMap<&str, String> = HashMap::new();
    output.insert("runtime", format!("Tokio Runtime: {:?}", RUNTIME.handle()));
    output.insert("database_state", format!("{:?}", database::get_state()));
    output
}
