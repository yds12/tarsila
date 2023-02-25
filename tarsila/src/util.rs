use std::path::PathBuf;
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
        match self.start {
            None => true,
            Some(t) => t.elapsed().unwrap().as_millis() as u64 > self.duration,
        }
    }
}

pub fn save_project(path: PathBuf, bytes: Vec<u8>) {
    use std::io::Write;
    let mut file = std::fs::File::create(path).unwrap();
    file.write(&bytes);
}

pub fn load_project(path: PathBuf) -> Vec<u8> {
    use std::io::Read;
    let mut file = std::fs::File::open(path).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    bytes
}
