use std::time::{Duration, Instant};

pub struct Timer {
    pub start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
