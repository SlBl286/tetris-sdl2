use std::time::{Duration, Instant};

pub struct FrameManager {
    target_frame_duration: Duration,
    last_frame: Instant,
}

pub trait FrameManagerTrait {
    fn new(target_fps: u32) -> Self;
    fn get_target_fps(&mut self) -> f32;
    fn delta_time(&mut self) -> Duration;
    fn delay_to_maintain_fps(&mut self);
}

impl FrameManagerTrait for FrameManager {
    fn new(target_fps: u32) -> Self {
        Self {
            target_frame_duration: Duration::from_secs_f64(1.0 / target_fps as f64),
            last_frame: Instant::now(),
        }
    }
    fn get_target_fps(&mut self) -> f32 {
        self.target_frame_duration.as_secs_f32()
    }
    fn delay_to_maintain_fps(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame);

        if elapsed < self.target_frame_duration {
            std::thread::sleep(self.target_frame_duration - elapsed);
        }

        self.last_frame = Instant::now();
    }

     fn delta_time(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;
        return delta;
    }
}
