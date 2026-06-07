# FlowWire Routing Engine

The routing engine is the heart of FlowWire. It maintains a graph of nodes and edges representing media sources, sinks, and processing units.

## Graph Model

- **Node**: Represents a source, sink, or processing unit
  - Example: Microphone, Audio Output, Compressor, Virtual Camera
- **Edge**: Represents a connection from output ports to input ports
[Node A Output] --> [Node B Input]


## Scheduling

- Nodes are processed in topological order
- Real-time priority is used for audio/video threads
- Buffers are managed to minimize latency and prevent dropouts

## Dynamic Routing

- Nodes and edges can be added or removed at runtime
- GUI updates reflect changes in real-time
- Supports fan-out, fan-in, and feedback loops

## Plugins

- Audio/video processing nodes can be provided by plugins
- API allows adding new node types without restarting the daemon
