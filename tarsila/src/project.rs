use crate::VERSION;
use std::path::PathBuf;

const MAGIC: [u8; 5] = [0xfa, 0x1a, 0xfe, 0x1b, 0xee];

pub fn save(path: PathBuf, bytes: Vec<u8>) {
    use std::io::Write;
    let mut file = std::fs::File::create(path).unwrap();
    let header = header();
    file.write(&header).unwrap();
    file.write(&bytes).unwrap();
}

pub fn load(path: PathBuf) -> Vec<u8> {
    use std::io::Read;
    let mut file = std::fs::File::open(path).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    without_header(bytes)
}

fn header() -> Vec<u8> {
    let mut bytes = format!("tarsila {VERSION} ").into_bytes();
    bytes.append(&mut MAGIC.as_slice().to_owned());

    bytes
}

fn without_header(mut bytes: Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    let mut found = false;
    for win in bytes.windows(MAGIC.len()) {
        if win == MAGIC {
            found = true;
            break;
        }

        i += 1;
    }

    if found {
        bytes.split_off(i + MAGIC.len())
    } else {
        bytes
    }
}
