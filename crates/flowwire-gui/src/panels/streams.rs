use eframe::egui::{Ui, CollapsingHeader};

pub struct StreamsPanel;

impl StreamsPanel {
    pub fn show(ui: &mut Ui) {
        CollapsingHeader::new("Streams")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Audio Stream: Mic -> Speakers");
                ui.label("MIDI Stream: Keyboard -> Synth");
                ui.label("Video Stream: Webcam -> Virtual Camera");
            });
    }
}
