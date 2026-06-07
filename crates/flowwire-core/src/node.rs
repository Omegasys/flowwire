use serde::{Serialize, Deserialize};

/// Represents a node in the graph (source, sink, or processor)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
    pub name: String,
    pub node_type: NodeType,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NodeType {
    AudioInput,
    AudioOutput,
    MidiInput,
    MidiOutput,
    VideoInput,
    VideoOutput,
    VirtualInput(String), // e.g., file or virtual device
}

impl Node {
    pub fn new(name: &str, node_type: NodeType) -> Self {
        Self {
            name: name.to_string(),
            node_type,
        }
    }
}
