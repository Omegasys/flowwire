# FlowWire Developer Guide

This guide helps developers contribute to FlowWire or create plugins.

## Getting Started

```bash
# Clone repository
git clone https://github.com/username/flowwire.git
cd flowwire

# Build all crates
cargo build --workspace

Crate Overview
flowwire-core: graph and event engine
flowwire-daemon: main service
flowwire-gui: visual frontend
flowwire-audio/midi/video: media handlers
flowwire-plugins: plugin loader
Creating a Plugin
Create a new Rust crate
Implement the FlowWirePlugin trait
Register nodes and processing functions
Compile and place in plugins/
Load plugin through GUI or IPC command
Testing
Unit tests in each crate
Integration tests in tests/
Benchmark with scripts/benchmark.sh
Contribution Guidelines
Follow Rust formatting conventions
Document all public APIs
Use CI workflows in .github/workflows/ for testing
Open pull requests with detailed descriptions
