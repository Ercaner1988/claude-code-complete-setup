#!/usr/bin/env bash
set -e

echo -e "\033[0;34m[INFO]\033[0m Bootstrapping Claude Code Rust Engine..."

# Build the Rust binary in release mode
cargo build --release

# Determine executable name based on OS
if [ -f "./target/release/claude-code-setup.exe" ]; then
    EXEC="./target/release/claude-code-setup.exe"
else
    EXEC="./target/release/claude-code-setup"
fi

# Forward to the Rust engine
exec "$EXEC" update "$@"
