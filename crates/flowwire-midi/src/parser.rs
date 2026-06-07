/// MIDI message parser (very simplified)
#[derive(Debug, Clone)]
pub struct MidiMessage {
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
}

pub struct MidiParser;

impl MidiParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse raw bytes into MIDI message
    pub fn parse(&self, bytes: &[u8]) -> Option<MidiMessage> {
        if bytes.len() < 3 {
            return None;
        }

        Some(MidiMessage {
            status: bytes[0],
            data1: bytes[1],
            data2: bytes[2],
        })
    }
}
