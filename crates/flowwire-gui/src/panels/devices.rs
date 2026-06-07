use eframe::egui::{Ui, CollapsingHeader};

pub struct DevicesPanel;

impl DevicesPanel {
    pub fn show(ui: &mut Ui) {
        CollapsingHeader::new("Devices")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Audio Input: Microphone");
                ui.label("Audio Output: Speakers");
                ui.label("MIDI Input: Keyboard");
                ui.label("Video Input: Webcam");
            });
    }
}
