/// Combines multiple video sources into one output frame
pub struct Compositor;

impl Compositor {
    pub fn new() -> Self {
        Self
    }

    pub fn composite(&self, inputs: Vec<Vec<u8>>) -> Vec<u8> {
        let mut output = Vec::new();

        for input in inputs {
            output.extend(input);
        }

        output
    }
}
