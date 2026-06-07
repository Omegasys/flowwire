#!/usr/bin/env bash

set -e

echo "Installing FlowWire..."

cargo build --release --workspace

sudo install -Dm755 \
target/release/flowwire-daemon \
/usr/local/bin/flowwire-daemon

sudo install -Dm644 \
systemd/flowwire.service \
/etc/systemd/system/flowwire.service

sudo install -Dm644 \
systemd/flowwire.socket \
/etc/systemd/system/flowwire.socket

sudo mkdir -p /usr/local/share/flowwire/configs

sudo cp configs/*.toml \
/usr/local/share/flowwire/configs/

sudo systemctl daemon-reload

echo "Installation complete."
echo "Enable with:"
echo "  sudo systemctl enable --now flowwire.socket"
