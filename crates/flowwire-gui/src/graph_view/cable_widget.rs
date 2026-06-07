use eframe::egui::{Painter, Pos2, Color32};

pub struct CableWidget;

impl CableWidget {
    pub fn draw(painter: &Painter, from: Pos2, to: Pos2) {
        painter.line_segment([from, to], (2.0, Color32::YELLOW));
    }
}
