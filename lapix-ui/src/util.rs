use std::time::SystemTime;

pub struct Timer {
    start: Option<SystemTime>,
    duration: u64
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: None,
            duration: 0
        }
    }

    pub fn start(&mut self, duration: u64) {
        self.start = Some(SystemTime::now());
        self.duration = duration;
    }

    pub fn expired(&self) -> bool {
        match self.start {
            None => true,
            Some(t) => t.elapsed().unwrap().as_millis() as u64 > self.duration
        }
    }
}
