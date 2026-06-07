use eframe::egui::{Ui, CollapsingHeader, ProgressBar};

pub struct LatencyPanel;

impl LatencyPanel {
    pub fn show(ui: &mut Ui) {
        CollapsingHeader::new("Latency")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Audio Latency");
                ui.add(ProgressBar::new(0.05).text("50ms"));
                ui.label("MIDI Latency");
                ui.add(ProgressBar::new(0.02).text("20ms"));
                ui.label("Video Latency");
                ui.add(ProgressBar::new(0.1).text("100ms"));
            });
    }
}
