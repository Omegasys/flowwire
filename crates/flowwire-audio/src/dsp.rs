/// Basic DSP utilities (filters, effects, etc.)
pub struct Dsp;

impl Dsp {
    pub fn new() -> Self {
        Self
    }

    /// Simple gain adjustment
    pub fn gain(&self, input: &[f32], gain: f32) -> Vec<f32> {
        input.iter().map(|s| s * gain).collect()
    }

    /// Very simple soft clipper
    pub fn soft_clip(&self, input: &[f32]) -> Vec<f32> {
        input
            .iter()
            .map(|&x| (x.tanh())) // crude saturation
            .collect()
    }

    /// DC offset removal (very naive high-pass)
    pub fn remove_dc(&self, input: &[f32]) -> Vec<f32> {
        let mut output = Vec::with_capacity(input.len());
        let mut last = 0.0;

        for &sample in input {
            let filtered = sample - last * 0.995;
            output.push(filtered);
            last = sample;
        }

        output
    }
}
