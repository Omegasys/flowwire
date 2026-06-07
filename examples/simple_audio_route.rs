use flowwire_audio::{AudioInput, AudioOutput, Node, Graph};

fn main() -> anyhow::Result<()> {
    println!("Starting simple audio routing example...");

    // Create the graph
    let mut graph = Graph::new();

    // Add input node (microphone)
    let mic = Node::new("Microphone", AudioInput::default());
    graph.add_node(mic.clone());

    // Add output node (speakers)
    let speakers = Node::new("Speakers", AudioOutput::default());
    graph.add_node(speakers.clone());

    // Connect microphone to speakers
    graph.connect(&mic, "out", &speakers, "in")?;

    println!("Routing microphone -> speakers");
    
    // Start processing
    graph.start()?;
    
    // Keep running until user interrupts
    std::thread::park();

    Ok(())
}
