# FlowWire GUI Design

The GUI provides a live, interactive representation of the FlowWire routing graph.

## Core Components

1. **Canvas**
   - Displays nodes and edges
   - Supports zoom, pan, and drag
2. **Node Widgets**
   - Shows input/output ports, stream labels, and current state
   - Visual indicators for audio level, buffer usage, and latency
3. **Edge Widgets**
   - Animated cables show data flow direction
   - Color-coded by stream type (audio/MIDI/video)
4. **Panels**
   - Device panel: lists all available sources/sinks
   - Stream panel: shows active streams and plugins
   - Log panel: debug messages, latency warnings

## Themes

- Dark and light modes
- Adjustable colors for nodes and edges
- Optional custom themes from configuration files

## Interaction

- Click to select nodes/edges
- Drag to create or remove connections
- Right-click for context menus and plugin configuration
- Live updates without blocking GUI thread
