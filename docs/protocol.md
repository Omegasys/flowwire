# FlowWire IPC Protocol

FlowWire uses a custom IPC protocol to communicate between the daemon, GUI, and clients.

## Transport

- Unix domain sockets (`/run/flowwire.sock`) on Linux
- Optional TCP for remote connections

## Message Types

- **RegisterNode**: Adds a new node to the graph
- **UpdateEdge**: Creates or updates connections between nodes
- **StreamData**: Sends audio/video/MIDI buffers
- **QueryTopology**: Requests the current graph state
- **PluginCommand**: Load, unload, or configure a plugin
- **Heartbeat**: Keep-alive messages for monitoring

## Serialization

- Messages are serialized using **bincode** or **CBOR**
- Includes a simple checksum for data integrity

## Security

- Only local processes are allowed by default
- Socket permissions are enforced
- Optional authentication for remote connections
