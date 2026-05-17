# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Skua Armory is an Arma 3 mod (mod prefix `z\skua`) built with [HEMTT](https://hemtt.dev). It bundles a large set of mission-side tweaks, ACE/ZEN integrations, equipment retextures, and a custom Rust extension for things that can't be done in SQF (UUIDs, PostgreSQL access, mission file parsing).

## Commands

Mod builds (run from repo root):
- `hemtt dev` — fast dev build, symlinks into Arma's mod folder.
- `hemtt build` — full build, output in `.hemttout/build/`. This is what CI runs on every push/PR.
- `hemtt release` — release build for Steam Workshop upload (triggered by `v*.*.*` tags).

Extension (Rust, in `extension/`):
- Linux dedicated server: `cd extension && cargo build --release` → produces `target/x86_64-unknown-linux-gnu/release/libskua.so`.
- Windows: `cd extension && cargo xwin build --release --target x86_64-pc-windows-msvc` (needs `cargo install xwin --locked` and `rustup target add x86_64-pc-windows-msvc` once).
- Convenience: `extension/install_deps.sh` then `extension/build.sh`.
- After building, move the artifact to repo root as `skua_x64.dll` / `skua_x64.so` (the `_x64` suffix is required by Arma's extension loader).
- Tests: `cd extension && cargo test`. Integration tests (`src/database/tests.rs`) spin up Postgres via `testcontainers`, so they require Docker / Podman to be running. Unit tests (`sanitize_key`, the SQF↔Rust enum sync check) have no external dependencies.

Pre-commit/CI checks:
- `bash tools/check_debug_macros.sh` — fails if `DEBUG_MODE_FULL`, `DISABLE_COMPILE_CACHE`, or `ENABLE_PERFORMANCE_COUNTERS` are left uncommented anywhere in `.cpp/.hpp/.sqf`. The `Debug Check` GitHub Action runs this on every PR; comment those `#define`s out before merging.
- `cargo test` (via the `Extension` workflow) — runs unit + integration tests on every PR that touches `extension/`. Includes a sync check that fails if `addons/main/script_macros_enums.hpp` drifts from the Rust `DatabaseState` enum.

Bumping the version: edit `addons/main/script_version.hpp` (MAJOR/MINOR/PATCHLVL/BUILD). Every version bump must be accompanied by a GitHub release — push a `vMAJOR.MINOR.PATCHLVL.BUILD` tag (triggers `release.yml`, which runs `hemtt release` and uploads to the Steam Workshop) and publish a matching GitHub Release.

## Architecture

### Addons (`addons/<component>/`)
Each component is its own PBO, following ACE3 conventions:
- `$PBOPREFIX$` — sets PBO path to `z\skua\addons\<component>`.
- `script_component.hpp` — defines `COMPONENT`, optional `COMPONENT_BEAUTIFIED`, then `#include "\z\skua\addons\main\script_mod.hpp"` (which pulls in version constants and `script_macros.hpp`).
- `config.cpp` — `CfgPatches`, includes `CfgEventHandlers.hpp`, possibly `CfgSettings.hpp` (CBA settings) and `CfgFunctions.hpp`.
- `XEH_preInit.sqf` / `XEH_postInit.sqf` / `XEH_preStart.sqf` — CBA Extended Event Handlers entry points. `XEH_preInit.sqf` typically just does `PREP_RECOMPILE_START; #include "XEH_PREP.hpp"; PREP_RECOMPILE_END;`.
- `XEH_PREP.hpp` — list of `PREP(funcName);` declarations; each maps to `functions/fnc_funcName.sqf`.
- `functions/` — function bodies; called as `[args] call FUNC(name)` (i.e. `skua_<component>_fnc_name`).

`addons/main/` is the version/dependency hub — it owns `script_mod.hpp`, `script_version.hpp`, `script_macros.hpp`, `script_macros_enums.hpp`, and the `REQUIRED_VERSION` / `REQUIRED_CBA_VERSION` / `REQUIRED_ACE_VERSION` constants. The CBA Versioning system is wired in `main/CfgSettings.hpp` and will refuse to load on mismatched dependency versions.

### Macros
`addons/main/script_macros.hpp` is the cross-mod reference layer. Use it instead of hardcoding paths/classnames:
- `ACEFUNC(module, fn)`, `QACEGVAR(module, var)`, `ACEPATHTOF(...)` — refer to ACE3 code/vars.
- `ZENFUNC(module, fn)`, `ZENGVAR(...)`, `ZENPATHTOF(...)` — refer to Zeus Enhanced.
- `CLASS(name)` → `skua_name`; `FUNC(name)` (from ACE main) → `skua_<component>_fnc_name`.
- Slot constants `SLOT_GOGGLES`, `SLOT_HEADGEAR`, etc.

`addons/main/script_macros_enums.hpp` defines constants that must stay in sync with the Rust extension's enums (e.g. `DATABASESTATE_*` ↔ `DatabaseState`). The sync is enforced by a `cargo test` in `extension/src/database/tests.rs` — don't edit one side without the other.

### Native extension (`extension/`)
Rust + arma-rs. Entry point `src/lib.rs` registers:
- `descriptionExt` (`editor::description_ext`) — writes a mission `description.ext` template if absent.
- `uuid` — UUIDv7 generator.
- `diagnostics` — runtime + database state snapshot.
- `logger` group — `set_level`/`get_level` for dynamic tracing-level control.
- `database` group — PostgreSQL via `deadpool-postgres` / `tokio-postgres`, used by the `persistence` addon.
- `certification` group — `list`/`get_player`/`grant`/`revoke` for player certifications. Definitions live in `database/migrations/certifications/*.json`.
- `ranks` group — `list`/`get_player`/`set_player` for player ranks. Definitions live in `database/migrations/ranks/*.json`.

Module layout under `src/`:
- `core/` — global `tokio::Runtime` via `LazyLock`. Spawn async work onto `RUNTIME`.
- `error/` — unified `DbError`, `QueryResult`/`QueryState`/`QueryOutcome<T>`, and the `transient_error`/`transient_query_error` helpers (log + return a TransientFailure with caller location).
- `logging/` — `ArmaLayer` (a `tracing_subscriber::Layer` that forwards events to Arma via the `skua_ext_log` callback) plus `logger:set_level`/`get_level` commands. Initialized once at extension build with `logging::init(ctx)`.
- `database/` — split into `state.rs` (atomic `DatabaseState`), `pool.rs` (deadpool init from `DATABASE_*` env vars, lazy `OnceCell`), `schema.rs` (`bootstrap_schema` creates tables + seeds default rank; `bootstrap_master` adds cert/rank migration on top), `sql.rs` (`include_str!` of `sql/*.sql`), `commands.rs` (Arma group: `bootstrap`, `state`).
- `database/sql/` — numbered SQL files (000–099 = master, 100–199 = campaign templates with `${campaign_id}` placeholder).
- `domain/` — `PlayerId(u64)` with FromArma/IntoArma/ToSql so commands can take `PlayerId` directly.
- `certification/` + `ranks/` — each module is `commands.rs` (Arma entry points) + `migration.rs` (file-driven reconciliation) + `types.rs` + `tests.rs`. `*_inner` functions take a raw `tokio_postgres::Client` for direct integration testing.

SQF callers use `"skua" callExtension [<command>, <args>]`. The Arma callback channel for the database group is `skua:database` (e.g. `skua:database / bootstrap`); cert/rank callbacks land on `skua:certification` / `skua:ranks`. The logger emits on `skua_ext_log`.

### File-driven migrations (`database/migrations/`)
Cert and rank definitions are authored as JSON files. They're embedded into the extension binary at compile time via `include_dir!` and reconciled with the DB on every bootstrap:
1. UPSERT every file's contents.
2. DELETE DB rows that no longer have a backing file AND whose `created_at` predates `skua_master.migration_state.last_migration_at` (in-game additions after the last sync are preserved).
3. Bump `last_migration_at`.

Editing the migration files requires rebuilding the extension binary. No HEMTT side files needed at runtime.

Default rank `(0, 'Unranked')` is bootstrap-seeded so `player_info.rank`'s FK default is always satisfied even on a cold start with no rank files.

Database state constants (`DATABASESTATE_*`) in `addons/main/script_macros_enums.hpp` are checked at runtime as strings (not parsed numbers) — see comment at top of that file. The enum_sync test in `extension/src/database/tests.rs` enforces consistency.

### Persistence vs Session
- `addons/persistence/` — long-lived state stored in Postgres via the extension (loadout, medical state, position, per-map).
- `addons/session/` — in-memory state for the current mission only (e.g. reconnect-to-same-position), no database.

### Includes (`include/`)
Stubbed headers for external mods (ACE under `include/z/ace`, CBA under `include/x/cba`, Apex/Oldman data under `include/a3/`, S.O.G. Prairie Fire under `include/vn/`). These let HEMTT preprocess without needing those mods installed. Don't put real mod code here.

### Local Postgres (`database/`)
`compose.yaml` spins up Postgres on `127.0.0.1:55432` for ad-hoc local testing. The cargo integration tests don't need it — they manage their own containers via `testcontainers`.

## Conventions

- Patch notes / changelogs aren't maintained — `README.md` is mostly a "don't bother me" notice. Commit messages are the source of truth; follow conventional-commits style (`feat:`, `fix:`, `chore:`, `chore: prepare build X.Y.Z.B` for version bumps).
- Don't enable debug macros in committed code (see CI check above).
- New components: copy the structure of an existing minimal addon (e.g. `session/` or `common/`), update `script_component.hpp`'s `COMPONENT`, and add the addon name to anywhere it's referenced (HEMTT auto-discovers folders under `addons/`).
