#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$SCRIPT_DIR"

# Install cargo-xwin sequentially before forking — `cargo install` would race
# with the concurrent builds for the registry lock otherwise.
if ! command -v cargo-xwin >/dev/null 2>&1; then
    echo "Installing cargo-xwin"
    cargo install cargo-xwin --locked
fi

if command -v sccache >/dev/null 2>&1; then
    export RUSTC_WRAPPER=sccache
fi

echo "Building binaries for Windows and Linux concurrently"

# Capture build output to temp logs so the two streams don't interleave on the
# terminal. We dump them in order at the end, regardless of success/failure.
linux_log="$(mktemp)"
windows_log="$(mktemp)"
trap 'rm -f "$linux_log" "$windows_log"' EXIT

# Use a separate target dir so host-compiled build scripts don't leak into the
# cross container (glibc mismatch).
(CARGO_TARGET_DIR="$REPO_ROOT/target-cross" cross build --target x86_64-unknown-linux-gnu --release) >"$linux_log" 2>&1 &
linux_pid=$!

(cargo xwin build --release --target x86_64-pc-windows-msvc) >"$windows_log" 2>&1 &
windows_pid=$!

linux_status=0
windows_status=0
wait "$linux_pid" || linux_status=$?
wait "$windows_pid" || windows_status=$?

echo "===== Linux build output ====="
cat "$linux_log"
echo "===== Windows build output ====="
cat "$windows_log"

if [[ $linux_status -ne 0 ]]; then
    echo "Linux build failed (exit $linux_status)" >&2
fi
if [[ $windows_status -ne 0 ]]; then
    echo "Windows build failed (exit $windows_status)" >&2
fi
if [[ $linux_status -ne 0 || $windows_status -ne 0 ]]; then
    exit 1
fi

echo "Moving compiled binaries to repository root"

mv "$REPO_ROOT/target/x86_64-pc-windows-msvc/release/skua.dll" "$REPO_ROOT/skua_x64.dll"
mv "$REPO_ROOT/target-cross/x86_64-unknown-linux-gnu/release/libskua.so" "$REPO_ROOT/skua_x64.so"
