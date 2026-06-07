use crate::parser::MidiMessage;

/// Represents a physical or logical MIDI device
pub struct MidiDevice {
    pub name: String,
    pub connected: bool,
}

impl MidiDevice {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            connected: false,
        }
    }

    pub fn connect(&mut self) {
        self.connected = true;
        log::info!("MIDI device connected: {}", self.name);
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        log::info!("MIDI device disconnected: {}", self.name);
    }

    pub fn send(&self, msg: &MidiMessage) {
        if self.connected {
            log::debug!("Sending MIDI message from {}: {:?}", self.name, msg);
        }
    }
}
