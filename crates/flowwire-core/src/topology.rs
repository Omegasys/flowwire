use crate::node::Node;
use crate::edge::Edge;

/// Helper functions for analyzing the graph topology
pub fn node_connections(node: &Node, edges: &[Edge]) -> Vec<&Edge> {
    edges.iter()
        .filter(|e| e.from_node == node.name || e.to_node == node.name)
        .collect()
}
