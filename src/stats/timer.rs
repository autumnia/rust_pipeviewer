use std::time::{Duration, Instant};

pub struct Timer {
    pub end_instant: Instant,
    pub delta: Duration,
    pub period: Duration,
    pub countdown: Duration,
    pub ready: bool,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            end_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.end_instant;
        self.end_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
