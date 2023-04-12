use std::time::SystemTime;

pub fn rgba_to_rgb_u8(color: [u8; 4]) -> [u8; 3] {
    [color[0], color[1], color[2]]
}

pub struct Timer {
    start: Option<SystemTime>,
    duration: u64,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: None,
            duration: 0,
        }
    }

    pub fn start(&mut self, duration: u64) {
        self.start = Some(SystemTime::now());
        self.duration = duration;
    }

    pub fn expired(&self) -> bool {
        if let Some(dur) = self.start.and_then(|t| t.elapsed().ok()) {
            return dur.as_millis() as u64 > self.duration;
        }

        true
    }
}
