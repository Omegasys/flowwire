/// Simple camera abstraction (webcam or virtual camera input)
pub struct Camera {
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn start(&self) {
        log::info!("Camera started {}x{}", self.width, self.height);
    }

    /// Fake frame generator (replace with V4L2 / ffmpeg later)
    pub fn capture_frame(&self) -> Vec<u8> {
        vec![0; (self.width * self.height * 3) as usize]
    }
}
