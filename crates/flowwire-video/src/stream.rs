use crate::camera::Camera;

/// Represents a video stream pipeline
pub struct VideoStream {
    pub source: Camera,
}

impl VideoStream {
    pub fn new(source: Camera) -> Self {
        Self { source }
    }

    pub fn next_frame(&self) -> Vec<u8> {
        self.source.capture_frame()
    }
}
