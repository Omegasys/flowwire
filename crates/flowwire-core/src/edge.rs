/// Represents a connection between two nodes
#[derive(Debug)]
pub struct Edge {
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}

impl Edge {
    pub fn new(from_node: String, from_port: String, to_node: String, to_port: String) -> Self {
        Self {
            from_node,
            from_port,
            to_node,
            to_port,
        }
    }
}
