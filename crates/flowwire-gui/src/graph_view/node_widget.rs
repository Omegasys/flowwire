use eframe::egui::{Painter, Pos2, Rect, Color32};
use flowwire_core::Node;

pub struct NodeWidget;

impl NodeWidget {
    pub fn draw(_ui: &eframe::egui::Ui, painter: &Painter, node: &Node) {
        // Simple static position for demo
        let pos = Pos2::new(100.0, 100.0);
        let size = 80.0;

        let rect = Rect::from_min_size(pos, eframe::egui::vec2(size, size));
        painter.rect(rect, 5.0, Color32::from_rgb(60, 120, 200), Color32::BLACK);

        painter.text(
            pos + eframe::egui::vec2(10.0, 30.0),
            eframe::egui::Align2::LEFT_CENTER,
            &node.name,
            eframe::egui::FontId::default(),
            Color32::WHITE,
        );
    }
}
