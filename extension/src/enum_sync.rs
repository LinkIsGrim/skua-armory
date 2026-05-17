//! Cross-language enum sync check.
//!
//! Verifies that `addons/main/script_macros_enums.hpp` stays in sync with every
//! Rust enum whose discriminants the SQF side compares against. If you add,
//! rename, or reorder a variant on either side, run
//! `cargo test enum_sync -- --nocapture` to see the expected file contents.

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
];

fn expected_hpp() -> String {
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

fn hpp_path() -> std::path::PathBuf {
    // CARGO_MANIFEST_DIR = .../extension
    let manifest = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(manifest)
        .parent()
        .expect("manifest dir has parent")
        .join("addons/main/script_macros_enums.hpp")
}

#[test]
fn sqf_macros_match_rust_enums() {
    let path = hpp_path();
    let actual =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
    let expected = expected_hpp();

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
