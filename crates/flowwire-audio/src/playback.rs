use anyhow::Result;

/// Audio output device abstraction (speakers, virtual sinks, etc.)
pub struct AudioPlayback {
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioPlayback {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        Self {
            sample_rate,
            channels,
        }
    }

    pub fn start(&self) -> Result<()> {
        log::info!(
            "AudioPlayback started ({} Hz, {} channels)",
            self.sample_rate,
            self.channels
        );
        Ok(())
    }

    pub fn write_frame(&self, frame: &[f32]) -> Result<()> {
        // Stub: would send to ALSA/PulseAudio/PipeWire sink
        log::debug!("Playing frame of size {}", frame.len());
        Ok(())
    }
}
