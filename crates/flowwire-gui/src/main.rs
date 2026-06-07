use eframe::egui;
use flowwire_gui::graph_view::{canvas::Canvas, node_widget::NodeWidget};
use flowwire_core::{Graph, Node, NodeType};

fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };

    eframe::run_native(
        "FlowWire GUI",
        options,
        Box::new(|_cc| Box::new(FlowWireApp::new())),
    )?;

    Ok(())
}

struct FlowWireApp {
    graph: Graph,
}

impl FlowWireApp {
    pub fn new() -> Self {
        let mut graph = Graph::new();

        // Example nodes for demo
        let mic = Node::new("Mic", NodeType::AudioInput);
        let speaker = Node::new("Speaker", NodeType::AudioOutput);
        graph.add_node(mic);
        graph.add_node(speaker);

        Self { graph }
    }
}

impl eframe::App for FlowWireApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FlowWire Routing Graph");
            Canvas::show(ui, &self.graph);
        });
    }
}
