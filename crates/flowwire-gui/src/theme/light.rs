use eframe::egui::{Visuals, Color32};

pub fn light_theme() -> Visuals {
    let mut visuals = Visuals::light();
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(240, 240, 240);
    visuals.widgets.active.bg_fill = Color32::from_rgb(200, 200, 200);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(220, 220, 220);
    visuals
}
