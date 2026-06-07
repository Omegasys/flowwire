use crate::parser::MidiMessage;
use std::collections::HashMap;

/// Routes MIDI messages between ports/devices
pub struct MidiRouter {
    routes: HashMap<String, Vec<String>>,
}

impl MidiRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, from: &str, to: &str) {
        self.routes
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
    }

    pub fn route(&self, from: &str, msg: &MidiMessage) {
        if let Some(destinations) = self.routes.get(from) {
            for dest in destinations {
                log::debug!("Routing MIDI {:?} -> {}", msg, dest);
            }
        }
    }
}
