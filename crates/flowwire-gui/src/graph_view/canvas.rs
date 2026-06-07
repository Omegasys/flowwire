use eframe::egui::{Ui, Frame, Painter, Rect};
use flowwire_core::{Graph, Node};
use super::node_widget::NodeWidget;

pub struct Canvas;

impl Canvas {
    pub fn show(ui: &mut Ui, graph: &Graph) {
        Frame::canvas(ui.style()).show(ui, |ui| {
            let painter = ui.painter_at(ui.max_rect());

            // Draw nodes
            for node in graph.nodes.values() {
                NodeWidget::draw(ui, painter, node);
            }

            // TODO: Draw cables/edges
        });
    }
}
