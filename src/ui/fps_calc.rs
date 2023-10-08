pub struct FpsCalc {
    fps: usize,
    last_frame_time: std::time::Instant,
    frame: usize,
}

impl FpsCalc {
    pub fn new(fps: usize) -> Self {
        Self {
            fps,
            last_frame_time: std::time::Instant::now(),
            frame: 0,
        }
    }

    pub fn frame(&self) -> usize {
        self.frame
    }

    pub fn update(&mut self) -> Option<f32> {
        self.frame += 1;
        if self.frame % self.fps != 0 {
            return None;
        }

        let now = std::time::Instant::now();
        let elapsed = now - self.last_frame_time;
        let fps = self.fps as f32 / elapsed.as_secs_f32();
        self.last_frame_time = now;
        Some(fps)
    }
}
