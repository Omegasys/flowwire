# FlowWire Security

FlowWire is designed with security and isolation in mind to prevent malicious or buggy plugins from compromising the system.

## IPC Security

- Unix domain sockets are restricted to the owning user by default
- Optional TLS for remote connections
- Authenticated API tokens for external clients

## Plugin Isolation

- Plugins run in the daemon process but are sandboxed
- Access to system resources is minimized
- Memory safety is enforced using Rust’s ownership system

## Data Integrity

- Stream data messages include checksums
- Optional encryption of buffers for remote streaming
- GUI cannot execute arbitrary code from plugin metadata

## Best Practices

- Always run the daemon with non-root privileges
- Limit plugin installation to trusted sources
- Use systemd socket activation for controlled access
