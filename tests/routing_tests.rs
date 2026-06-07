use flowwire_core::{Graph, Node};
use flowwire_core::node::NodeType;

#[test]
fn test_simple_route_creation() {
    let mut graph = Graph::new();

    let input = Node::new("Input", NodeType::AudioInput);
    let output = Node::new("Output", NodeType::AudioOutput);

    graph.add_node(input.clone());
    graph.add_node(output.clone());

    let result = graph.connect(&input, "out", &output, "in");

    assert!(result.is_ok());
}

#[test]
fn test_multiple_routes() {
    let mut graph = Graph::new();

    let a = Node::new("A", NodeType::AudioInput);
    let b = Node::new("B", NodeType::AudioOutput);
    let c = Node::new("C", NodeType::AudioOutput);

    graph.add_node(a.clone());
    graph.add_node(b.clone());
    graph.add_node(c.clone());

    assert!(graph.connect(&a, "out", &b, "in").is_ok());
    assert!(graph.connect(&a, "out", &c, "in").is_ok());
}
