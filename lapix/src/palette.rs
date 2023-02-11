use crate::{util, Color};

const MAX_PALETTE: usize = 200;

pub struct Palette(Vec<Color>);

impl Default for Palette {
    fn default() -> Self {
        Self(vec![
            Color::new(0, 0, 0, 255),       // BLACK
            Color::new(255, 255, 255, 255), // WHITE
            Color::new(255, 0, 0, 255),     // RED
            Color::new(255, 127, 0, 255),   // RED + YELLOW = ORANGE
            Color::new(255, 255, 0, 255),   // YELLOW
            Color::new(127, 255, 0, 255),   // GREEN + YELLOW
            Color::new(0, 255, 0, 255),     // GREEN
            Color::new(0, 255, 127, 255),   // GREEN + CYAN
            Color::new(0, 255, 255, 255),   // CYAN
            Color::new(0, 127, 255, 255),   // BLUE + CYAN
            Color::new(0, 0, 255, 255),     // BLUE
            Color::new(127, 0, 255, 255),   // BLUE + MAGENTA
            Color::new(255, 0, 255, 255),   // MAGENTA
            Color::new(255, 0, 127, 255),   // RED + MAGENTA
        ])
    }
}

impl Palette {
    pub fn from_file(path: &str) -> Self {
        let img = util::load_img_from_file(path);
        let mut palette = Vec::new();

        for (_, _, pixel) in img.enumerate_pixels() {
            let color = Color::new(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);

            if !palette.contains(&color) {
                palette.push(color);
            }

            if palette.len() >= MAX_PALETTE {
                break;
            }
        }

        Self(palette)
    }

    pub fn add_color(&mut self, color: Color) {
        if !self.0.contains(&color) {
            self.0.push(color)
        }
    }

    pub fn remove_color(&mut self, color: Color) {
        self.0.retain(|c| *c != color);
    }

    pub fn inner(&self) -> &[Color] {
        &self.0
    }
}
