/// Simple multi-stream audio mixer
pub struct Mixer {
    pub gain: f32,
}

impl Mixer {
    pub fn new() -> Self {
        Self { gain: 1.0 }
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    /// Mix multiple audio buffers into one
    pub fn mix(&self, inputs: &[Vec<f32>]) -> Vec<f32> {
        if inputs.is_empty() {
            return vec![];
        }

        let max_len = inputs.iter().map(|v| v.len()).max().unwrap_or(0);
        let mut output = vec![0.0; max_len];

        for input in inputs {
            for (i, sample) in input.iter().enumerate() {
                output[i] += sample * self.gain;
            }
        }

        output
    }
}
