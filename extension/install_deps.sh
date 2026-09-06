#!/bin/bash

echo "Installing cross for cross-compilation"
cargo install cross --locked

echo "Installing sccache for compilation caching"
cargo install sccache --locked

echo "Installing mold linker (if not already installed)"
if ! command -v mold >/dev/null 2>&1; then
    if command -v apt-get >/dev/null 2>&1; then
        sudo apt-get install -y mold
    elif command -v dnf >/dev/null 2>&1; then
        sudo dnf install -y mold
    fi
fi

echo "Installing Windows toolchain dependencies"
rustup target add x86_64-pc-windows-msvc