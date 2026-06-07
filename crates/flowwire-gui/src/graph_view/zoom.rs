use eframe::egui::{Ui, Sense};

pub struct Zoom;

impl Zoom {
    pub fn handle(ui: &mut Ui) -> f32 {
        let zoom = 1.0;
        if ui.input().zoom_delta() != 1.0 {
            return ui.input().zoom_delta();
        }
        zoom
    }
}
