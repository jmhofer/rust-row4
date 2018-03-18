use std::time::Duration;
use std::time::SystemTime;

pub struct Timer {
    start: SystemTime
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: SystemTime::now()
        }
    }

    pub fn reset(&mut self) {
        self.start = SystemTime::now();
    }

    pub fn elapsed_millis(&self) -> u64 {
        let elapsed = self.elapsed();
        elapsed.as_secs() * 1_000 + elapsed.subsec_nanos() as u64 / 1_000_000
    }

    pub fn elapsed_micros(&self) -> u64 {
        let elapsed = self.elapsed();
        elapsed.as_secs() * 1_000_000 + elapsed.subsec_nanos() as u64 / 1_000
    }

    pub fn elapsed_nanos(&self) -> u64 {
        let elapsed = self.elapsed();
        elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64
    }

    fn elapsed(&self) -> Duration {
        SystemTime::now().duration_since(self.start).unwrap()
    }
}
