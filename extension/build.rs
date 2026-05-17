//! Cargo build script.
//!
//! `include_dir!()` reads its target directory at macro expansion time, but
//! Cargo doesn't see those reads — so adding/removing/editing files under
//! `database/migrations/` won't, by itself, invalidate a cached build. The
//! result is silent: the binary keeps the embedded file set from the last
//! actual recompile, and new migration files appear to be ignored.
//!
//! These `rerun-if-changed` directives close that gap: any modification under
//! `database/migrations/` (recursive) forces Cargo to re-expand the macro on
//! the next build.

fn main() {
    println!("cargo:rerun-if-changed=../database/migrations");
}
