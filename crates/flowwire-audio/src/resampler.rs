/// Simple linear resampler (placeholder)
pub struct Resampler {
    pub input_rate: u32,
    pub output_rate: u32,
}

impl Resampler {
    pub fn new(input_rate: u32, output_rate: u32) -> Self {
        Self {
            input_rate,
            output_rate,
        }
    }

    /// Naive resampling (NOT production quality)
    pub fn process(&self, input: &[f32]) -> Vec<f32> {
        if self.input_rate == self.output_rate {
            return input.to_vec();
        }

        let ratio = self.output_rate as f32 / self.input_rate as f32;
        let new_len = (input.len() as f32 * ratio) as usize;

        let mut output = Vec::with_capacity(new_len);

        for i in 0..new_len {
            let src_index = (i as f32 / ratio) as usize;
            output.push(*input.get(src_index).unwrap_or(&0.0));
        }

        output
    }
}
