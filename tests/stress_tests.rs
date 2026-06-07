use flowwire_core::{Graph, Node};
use flowwire_core::node::NodeType;

#[test]
fn test_large_graph_creation() {
    let mut graph = Graph::new();

    for i in 0..1000 {
        graph.add_node(
            Node::new(
                &format!("Node{}", i),
                NodeType::AudioInput,
            )
        );
    }

    assert!(graph.start().is_ok());
}

#[test]
fn test_massive_connection_count() {
    let mut graph = Graph::new();

    let source = Node::new("Source", NodeType::AudioInput);

    graph.add_node(source.clone());

    for i in 0..500 {
        let node = Node::new(
            &format!("Sink{}", i),
            NodeType::AudioOutput,
        );

        graph.add_node(node.clone());

        assert!(
            graph
                .connect(&source, "out", &node, "in")
                .is_ok()
        );
    }
}
