use eframe::egui::{Visuals, Color32};

pub fn dark_theme() -> Visuals {
    let mut visuals = Visuals::dark();
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(30, 30, 30);
    visuals.widgets.active.bg_fill = Color32::from_rgb(60, 60, 60);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(50, 50, 50);
    visuals
}
