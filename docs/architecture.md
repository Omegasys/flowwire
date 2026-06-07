# FlowWire Architecture

FlowWire is a modular media routing system designed for Linux. It consists of several independent crates communicating through IPC, while providing a GUI for live visualization of audio, MIDI, and video streams.

## Core Components

1. **flowwire-core**
   - Handles the internal graph of nodes and edges.
   - Maintains the routing topology and event bus.
   - Scheduler manages timed events and buffer processing.

2. **flowwire-daemon**
   - Runs as a system service.
   - Manages nodes, streams, and device discovery.
   - Responsible for IPC between clients and plugins.

3. **flowwire-gui**
   - Displays the routing graph in real-time.
   - Provides controls for nodes, streams, and plugins.
   - Supports dark/light themes and zoomable canvas.

4. **flowwire-audio / flowwire-midi / flowwire-video**
   - Handle media-specific capture, playback, processing, and routing.
   - Provide APIs for plugins and external applications.

5. **flowwire-plugins**
   - Supports runtime-loadable plugins for effects, compression, noise suppression, and virtual devices.

## Data Flow


[Application] --> [GUI / API] --> [Daemon] --> [Core Graph] --> [Device]


- The daemon manages the internal graph, while the GUI provides a visual representation.
- Plugins and nodes are dynamically added or removed.

## Design Goals

- Modular and extensible
- Lightweight and efficient
- Real-time performance with minimal latency
- GUI-first visibility of all connections
