use anyhow::Result;

/// Audio capture device abstraction (microphone, system input, etc.)
pub struct AudioCapture {
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioCapture {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        Self {
            sample_rate,
            channels,
        }
    }

    /// Start capturing audio frames (stubbed)
    pub fn start(&self) -> Result<()> {
        log::info!(
            "AudioCapture started ({} Hz, {} channels)",
            self.sample_rate,
            self.channels
        );
        Ok(())
    }

    /// Fake capture frame (replace with ALSA/Pulse/PipeWire later)
    pub fn read_frame(&self) -> Vec<f32> {
        vec![0.0; (self.sample_rate as usize / 100)] // ~10ms buffer
    }
}
