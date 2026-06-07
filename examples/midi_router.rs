use flowwire_midi::{MidiInput, MidiOutput, Node, Graph};

fn main() -> anyhow::Result<()> {
    println!("Starting MIDI router example...");

    let mut graph = Graph::new();

    // Add MIDI input
    let midi_in = Node::new("MIDI Keyboard", MidiInput::default());
    graph.add_node(midi_in.clone());

    // Add MIDI output
    let midi_out = Node::new("MIDI Synth", MidiOutput::default());
    graph.add_node(midi_out.clone());

    // Connect input to output
    graph.connect(&midi_in, "out", &midi_out, "in")?;

    println!("Routing MIDI keyboard -> synthesizer");

    graph.start()?;

    std::thread::park();
    Ok(())
}
