#!/usr/bin/env bash

set -e

echo "Building FlowWire workspace..."

cargo fmt --all
cargo clippy --workspace --all-targets
cargo build --workspace

echo "Build complete."
