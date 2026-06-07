use crate::parser::MidiMessage;

/// Virtual MIDI port for software routing (DAW-like behavior)
pub struct VirtualMidiPort {
    pub name: String,
    buffer: Vec<MidiMessage>,
}

impl VirtualMidiPort {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            buffer: Vec::new(),
        }
    }

    pub fn push(&mut self, msg: MidiMessage) {
        self.buffer.push(msg);
    }

    pub fn drain(&mut self) -> Vec<MidiMessage> {
        let out = self.buffer.clone();
        self.buffer.clear();
        out
    }
}
