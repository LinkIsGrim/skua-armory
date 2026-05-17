#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$SCRIPT_DIR"

echo "Building binaries for Windows and Linux"

echo "Building Linux"
cross build --target x86_64-unknown-linux-gnu --release

echo "Building Windows"
if ! command -v cargo-xwin >/dev/null 2>&1; then
	cargo install cargo-xwin --locked
fi
cargo xwin build --release --target x86_64-pc-windows-msvc

echo "Moving compiled binaries to repository root"

mv "$REPO_ROOT/target/x86_64-pc-windows-msvc/release/skua.dll" "$REPO_ROOT/skua_x64.dll"
mv "$REPO_ROOT/target/x86_64-unknown-linux-gnu/release/libskua.so" "$REPO_ROOT/skua_x64.so"
