use flowwire_video::{Camera, VideoOutput, Node, Graph};

fn main() -> anyhow::Result<()> {
    println!("Starting video stream example...");

    let mut graph = Graph::new();

    // Video input (webcam)
    let cam = Node::new("Webcam", Camera::default());
    graph.add_node(cam.clone());

    // Video output (e.g., OBS or virtual camera)
    let vout = Node::new("Virtual Camera", VideoOutput::default());
    graph.add_node(vout.clone());

    // Connect camera to virtual output
    graph.connect(&cam, "out", &vout, "in")?;

    println!("Routing webcam -> virtual camera");

    graph.start()?;
    
    std::thread::park();
    Ok(())
}
