#!/usr/bin/env bash

set -e

echo "Running FlowWire benchmarks..."

cargo test --release --workspace

cargo bench || true

echo "Benchmark run complete."
