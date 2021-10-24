use std::time::Duration;
use std::time::Instant;

pub struct Clock {
    period: Duration,
    offset: Instant,
}

impl Clock {
    pub fn new(freq: f32) -> Self {
        Self {
            period: Duration::from_nanos(1_000_000_000 / freq as u64),
            offset: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.offset.elapsed() >= self.period {
            self.offset += self.period;
            true
        } else {
            false
        }
    }
}
