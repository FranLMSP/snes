use std::{thread, time};
use std::time::Instant;

pub struct FrameLimiter {
    timer: Instant,
    fps: u128,
}

impl FrameLimiter {
    pub fn new() -> Self {
        Self {
            timer: Instant::now(),
            fps: 16600,
        }
    }

    pub fn reset_timer(&mut self) {
        self.timer = Instant::now();
    }

    pub fn limit(&self) {
        let elapsed = self.timer.elapsed().as_micros();
        if elapsed > self.fps {
            return;
        }
        let wait = (self.fps - elapsed).try_into().unwrap();
        thread::sleep(time::Duration::from_micros(wait));
    }
}

impl Default for FrameLimiter {
    fn default() -> Self {
        Self::new()
    }
}
