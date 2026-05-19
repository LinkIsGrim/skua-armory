//! Cross-language enum sync check.
//!
//! Verifies that the Rust-side enums and event vocabulary stay in lockstep
//! with the SQF macros that depend on them. Two files are checked:
//!
//! - `addons/main/script_macros_enums.hpp` — `DATABASESTATE_*` and `QUERYSTATE_*`
//!   constants used as wire-format discriminants.
//! - `addons/main/script_macros_events.hpp` — `QEV_*` event-name constants used
//!   as CBA event names by every SQF subscriber.
//!
//! For events, an additional check enforces that each generated macro is
//! actually referenced by at least one `.sqf` file under `addons/`. This
//! catches the "added a variant in Rust, forgot to wire a SQF handler" class
//! of bug at `cargo test` time.
//!
//! If any check fails, run `cargo test enum_sync -- --nocapture` to see the
//! expected file contents in the test output.

use std::fmt::Write;

use crate::database::DatabaseState;
use crate::error::QueryState;

/// Database state — wire format is `String` (callExtension synchronous return
/// is always a string), so macros are quoted.
const DATABASE_STATE_VARIANTS: &[(&str, DatabaseState)] = &[
    ("AWAITCONNECT", DatabaseState::AwaitConnect),
    ("CONNECTEDINIT", DatabaseState::ConnectedInit),
    ("CONNECTEDAWAITINIT", DatabaseState::ConnectedAwaitInit),
    ("FAILED", DatabaseState::Failed),
];

/// Query state — wire format is `Number` and lands in callback payloads after
/// `parseSimpleArray`, so macros are bare scalars.
const QUERY_STATE_VARIANTS: &[(&str, QueryState)] = &[
    ("PROCESSING", QueryState::Processing),
    ("DONE", QueryState::Done),
    ("INVALIDARGUMENT", QueryState::InvalidArgument),
    ("TRANSIENTFAILURE", QueryState::TransientFailure),
    ("TIMEOUT", QueryState::Timeout),
];

/// Event-name macros consumed by SQF subscribers via `CBA_fnc_addEventHandler`.
/// Each entry is `(macro_suffix, event_name)` where the macro is `QEV_<suffix>`
/// and the value MUST equal `Event::event_name()` for the corresponding Rust
/// variant.
const EVENT_VARIANTS: &[(&str, &str)] = &[
    ("CERTIFICATION_GRANTED", "skua_certification_granted"),
    ("CERTIFICATION_REVOKED", "skua_certification_revoked"),
    (
        "CERTIFICATION_LIST_CHANGED",
        "skua_certification_list_changed",
    ),
    ("RANK_CHANGED", "skua_rank_changed"),
    ("RANK_LIST_CHANGED", "skua_rank_list_changed"),
    ("PLAYER_CONNECTED", "skua_player_connected"),
    ("PLAYER_DISCONNECTED", "skua_player_disconnected"),
];

fn expected_enums_hpp() -> String {
    let mut out = String::new();

    out.push_str(
        "// parseNumber is slower than comparing the string directly, so we'll just deal with it\n",
    );
    out.push_str("// these MUST match the Rust extension's DatabaseState enum (see extension/src/database/state.rs)\n");
    let width = DATABASE_STATE_VARIANTS
        .iter()
        .map(|(n, _)| n.len())
        .max()
        .unwrap_or(0);
    for (name, state) in DATABASE_STATE_VARIANTS {
        let _ = writeln!(
            out,
            "#define DATABASESTATE_{name:<width$} (\"{value}\")",
            name = name,
            width = width,
            value = *state as u8,
        );
    }

    out.push('\n');
    out.push_str(
        "// QueryState appears in callback payloads as a bare number (after parseSimpleArray),\n",
    );
    out.push_str(
        "// so these macros are scalars — match with `_state isEqualTo QUERYSTATE_DONE`.\n",
    );
    out.push_str("// these MUST match the Rust extension's QueryState enum (see extension/src/error/query.rs)\n");
    let width = QUERY_STATE_VARIANTS
        .iter()
        .map(|(n, _)| n.len())
        .max()
        .unwrap_or(0);
    for (name, state) in QUERY_STATE_VARIANTS {
        let _ = writeln!(
            out,
            "#define QUERYSTATE_{name:<width$} {value}",
            name = name,
            width = width,
            value = *state as u8,
        );
    }

    out
}

fn expected_events_hpp() -> String {
    let mut out = String::new();
    out.push_str("// Event-name macros for the unified skua:event channel.\n");
    out.push_str("// MUST match `Event::event_name()` in extension/src/event/mod.rs.\n");
    out.push_str(
        "// SQF subscribers register handlers via `[QEV_*, ...] call CBA_fnc_addEventHandler`.\n",
    );
    let width = EVENT_VARIANTS
        .iter()
        .map(|(n, _)| n.len())
        .max()
        .unwrap_or(0);
    for (name, value) in EVENT_VARIANTS {
        let _ = writeln!(out, "#define QEV_{name:<width$} \"{value}\"");
    }
    out
}

fn repo_root() -> std::path::PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(manifest)
        .parent()
        .expect("manifest dir has parent")
        .to_path_buf()
}

fn enums_hpp_path() -> std::path::PathBuf {
    repo_root().join("addons/main/script_macros_enums.hpp")
}

fn events_hpp_path() -> std::path::PathBuf {
    repo_root().join("addons/main/script_macros_events.hpp")
}

fn assert_file_matches(path: &std::path::Path, expected: &str) {
    let actual =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
    if actual != expected {
        eprintln!(
            "\n--- expected ({})\n{}\n--- actual\n{}\n",
            path.display(),
            expected,
            actual
        );
        panic!(
            "{} is out of sync with Rust enums. Replace its contents with the 'expected' block above.",
            path.display()
        );
    }
}

#[test]
fn sqf_macros_match_rust_enums() {
    assert_file_matches(&enums_hpp_path(), &expected_enums_hpp());
    assert_file_matches(&events_hpp_path(), &expected_events_hpp());
}

/// Walks every `.sqf` file under `addons/` and collects the text. Used by the
/// handler-existence check below. Returns one big concatenated blob — we only
/// care about substring presence, not file boundaries.
fn collect_addons_sqf_text() -> String {
    fn walk(dir: &std::path::Path, out: &mut String) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().and_then(|s| s.to_str()) == Some("sqf")
                && let Ok(text) = std::fs::read_to_string(&path)
            {
                out.push_str(&text);
                out.push('\n');
            }
        }
    }
    let mut out = String::new();
    walk(&repo_root().join("addons"), &mut out);
    out
}

/// For each `QEV_*` macro, confirm at least one `.sqf` file under `addons/`
/// references it. The realistic place to use these macros is in a
/// `CBA_fnc_addEventHandler` call, so this is a tight proxy for "an event has
/// a SQF subscriber". Prevents Rust-side variants from going un-wired.
#[test]
fn every_event_macro_has_a_subscriber() {
    let sqf = collect_addons_sqf_text();
    let mut missing: Vec<String> = Vec::new();
    for (name, _value) in EVENT_VARIANTS {
        let macro_ident = format!("QEV_{name}");
        if !sqf.contains(&macro_ident) {
            missing.push(macro_ident);
        }
    }
    assert!(
        missing.is_empty(),
        "Event macros without a SQF subscriber: {missing:?}\n\
         Add `[<macro>, LINKFUNC(handler)] call CBA_fnc_addEventHandler;` \
         in the consuming addon's XEH_preInit.sqf."
    );
}
