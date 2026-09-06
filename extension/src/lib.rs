//! Skua extension entry point.

use std::collections::HashMap;

use arma_rs::{Extension, arma};
use tracing::instrument;
use uuid::Uuid;

pub mod core;
pub use core::{CONTEXT, RUNTIME};

pub mod error;
pub use error::{DbError, QueryError, QueryResult, QueryState};

pub mod certification;
pub mod database;
pub mod domain;
pub mod editor;
pub mod event;
pub mod loadout;
pub mod logging;
pub mod player_info;
pub mod ranks;
pub mod sync;

pub use domain::PlayerId;

#[cfg(test)]
mod enum_sync;

#[arma]
fn init() -> Extension {
    let ext = Extension::build()
        // Top-level commands
        .command("descriptionExt", editor::description_ext) // "skua" callExtension ["descriptionExt", [getMissionPath ""]]
        .command("uuid", make_uuid) // "skua" callExtension ["uuid", []]
        .command("diagnostics", diagnostics)
        // Command groups
        .group("logger", logging::group())
        .group("database", database::group())
        .group("certification", certification::group())
        .group("loadout", loadout::group())
        .group("player_info", player_info::group())
        .group("ranks", ranks::group())
        .finish();

    let _ = CONTEXT.set(ext.context());
    logging::init(ext.context());

    // Surface panics through tracing so a panic in a spawned task doesn't die
    // silently. Without this, a panic inside `RUNTIME.spawn(...)` aborts the
    // task and we never see the cause (extension stdout/stderr isn't always
    // captured by Arma).
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        tracing::error!(panic = %info, "extension panic");
        default_hook(info);
    }));

    ext
}

#[instrument(level = "debug", name = "diagnostics", skip_all)]
fn diagnostics() -> HashMap<&'static str, String> {
    let mut output: HashMap<&str, String> = HashMap::new();
    output.insert("runtime", format!("Tokio Runtime: {:?}", RUNTIME.handle()));
    output.insert("database_state", format!("{:?}", database::get_state()));
    output
}

#[instrument(level = "debug", name = "make_uuid", skip_all)]
fn make_uuid() -> Uuid {
    Uuid::now_v7()
}
