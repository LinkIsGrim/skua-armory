#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$SCRIPT_DIR"

if command -v sccache >/dev/null 2>&1; then
    export RUSTC_WRAPPER=sccache
fi

if command -v mold >/dev/null 2>&1; then
    export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=-fuse-ld=mold"
fi

echo "Building Linux extension natively"
cargo build --release --locked

mv "$REPO_ROOT/target/release/libskua.so" "$REPO_ROOT/skua_x64.so"
echo "Installed skua_x64.so"
