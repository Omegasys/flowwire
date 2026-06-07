//! FlowWire Core
//!
//! Provides the graph, nodes, edges, event bus, and scheduling for FlowWire.
//! This crate is used by the daemon and GUI.

pub mod graph;
pub mod node;
pub mod edge;
pub mod scheduler;
pub mod event_bus;
pub mod topology;

pub use graph::Graph;
pub use node::Node;
pub use edge::Edge;
