use crate::node::Node;
use crate::edge::Edge;
use anyhow::Result;
use std::collections::HashMap;

/// Graph structure managing nodes and edges
#[derive(Default)]
pub struct Graph {
    nodes: HashMap<String, Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name.clone(), node);
    }

    pub fn connect(&mut self, from: &Node, from_port: &str, to: &Node, to_port: &str) -> Result<()> {
        let edge = Edge::new(from.name.clone(), from_port.to_string(), to.name.clone(), to_port.to_string());
        self.edges.push(edge);
        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        println!("Starting graph with {} nodes and {} edges", self.nodes.len(), self.edges.len());
        // TODO: Implement scheduling and streaming
        Ok(())
    }
}
