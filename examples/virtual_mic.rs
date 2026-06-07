use flowwire_audio::{VirtualInput, AudioOutput, Node, Graph};

fn main() -> anyhow::Result<()> {
    println!("Starting virtual mic example...");

    let mut graph = Graph::new();

    // Virtual input (acts like a microphone but software-generated)
    let virtual_mic = Node::new("Virtual Mic", VirtualInput::new("test.wav")?);
    graph.add_node(virtual_mic.clone());

    // Output (e.g., Discord, OBS, etc.)
    let app_output = Node::new("App Output", AudioOutput::default());
    graph.add_node(app_output.clone());

    // Connect virtual mic to output
    graph.connect(&virtual_mic, "out", &app_output, "in")?;

    println!("Routing virtual mic -> app output");

    graph.start()?;
    
    std::thread::park();
    Ok(())
}
