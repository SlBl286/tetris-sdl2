use std::time::{Duration, Instant};

pub struct FrameManager {
    target_frame_duration: Duration,
    last_frame: Instant,
}

impl FrameManager {
    pub fn new(target_fps: u32) -> Self {
        Self {
            target_frame_duration: Duration::from_secs_f64(1.0 / target_fps as f64),
            last_frame: Instant::now(),
        }
    }

    pub fn delay_to_maintain_fps(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame);

        if elapsed < self.target_frame_duration {
            std::thread::sleep(self.target_frame_duration - elapsed);
        }

        self.last_frame = Instant::now();
    }

    pub fn delta_time(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;
        return delta;
    }
}
