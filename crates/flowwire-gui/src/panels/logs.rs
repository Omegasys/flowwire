use eframe::egui::{Ui, ScrollArea};

pub struct LogsPanel;

impl LogsPanel {
    pub fn show(ui: &mut Ui, logs: &[String]) {
        ScrollArea::vertical().show(ui, |ui| {
            for log in logs {
                ui.label(log);
            }
        });
    }
}
