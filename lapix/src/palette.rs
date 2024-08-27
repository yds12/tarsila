use crate::{util, Color, Result};
use serde::{Deserialize, Serialize};

const MAX_PALETTE: usize = 200;

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn from_file(path: &str) -> Result<Self> {
        let img = util::load_img_from_file(path)?;

        Ok(Self::from_image(img))
    }

    fn from_image(img: image::RgbaImage) -> Self {
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
        let mut palette = Self(palette);
        palette.sort();

        palette
    }

    pub fn add_color(&mut self, color: Color) {
        if !self.0.contains(&color) {
            self.0.push(color)
        }
        self.sort();
    }

    pub fn remove_color(&mut self, color: Color) {
        self.0.retain(|c| *c != color);
    }

    pub fn colors(&self) -> &[Color] {
        &self.0
    }

    pub fn sort(&mut self) {
        fn sort_val(color: &Color) -> i32 {
            (color.hue() as i32) * 1_000_000
                + (color.saturation() * 10_000.) as i32
                + (color.value() * 10_000.) as i32
        }
        self.0.sort_by(|a, b| sort_val(a).cmp(&sort_val(b)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_bytes(bytes: Vec<u8>) -> Palette {
        let len = bytes.len() as u32 / 4;
        let img = image::RgbaImage::from_raw(1, len, bytes).unwrap();
        Palette::from_image(img)
    }

    #[test]
    fn create_from_img() {
        let bytes = vec![0, 0, 0, 255];
        let palette = from_bytes(bytes);
        assert!(palette.colors().contains(&Color::new(0, 0, 0, 255)));
        assert_eq!(palette.colors().len(), 1);

        let bytes = vec![0, 0, 0, 255, 0, 0, 0, 255];
        let palette = from_bytes(bytes);
        assert!(palette.colors().contains(&Color::new(0, 0, 0, 255)));
        assert_eq!(palette.colors().len(), 1);

        let bytes = vec![0, 0, 0, 255, 255, 0, 0, 255];
        let palette = from_bytes(bytes);
        assert!(palette.colors().contains(&Color::new(0, 0, 0, 255)));
        assert!(palette.colors().contains(&Color::new(255, 0, 0, 255)));
        assert_eq!(palette.colors().len(), 2);
    }

    #[test]
    fn add_and_remove_from_default() {
        let mut palette = Palette::default();

        let dark = Color::new(10, 10, 10, 255);
        palette.add_color(dark);
        assert!(palette.colors().contains(&dark));

        palette.remove_color(dark);
        assert!(!palette.colors().contains(&dark));
    }

    #[test]
    fn add_one() {
        let bytes = vec![0, 0, 0, 255];
        let mut palette = from_bytes(bytes);

        let color = Color::new(0, 1, 2, 3);
        palette.add_color(color);
        assert!(palette.colors().contains(&color));
        assert_eq!(palette.colors().len(), 2);
    }

    #[test]
    fn remove_one() {
        let bytes = vec![0, 0, 0, 255, 1, 1, 1, 255];
        let mut palette = from_bytes(bytes);

        let black = Color::new(0, 0, 0, 255);
        palette.remove_color(black);
        assert!(!palette.colors().contains(&black));
        assert_eq!(palette.colors().len(), 1);
    }
}
