use lapix::{Bitmap, Color};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct WrappedImage(pub Image);

impl Bitmap for WrappedImage {
    fn new(width: u16, height: u16, color: Color) -> Self {
        let bytes = vec![0; width as usize * height as usize * 4];
        let mut img = Self(Image {
            bytes,
            width,
            height,
        });

        for i in 0..width {
            for j in 0..height {
                img.set_pixel(i, j, color);
            }
        }

        img
    }

    fn width(&self) -> u16 {
        self.0.width() as u16
    }

    fn height(&self) -> u16 {
        self.0.height() as u16
    }

    fn pixel(&self, x: u16, y: u16) -> Color {
        let x = x as usize;
        let y = y as usize;
        let base_idx = y * 4 * self.width() as usize + x * 4;
        Color::new(
            self.0.bytes[base_idx],
            self.0.bytes[base_idx + 1],
            self.0.bytes[base_idx + 2],
            self.0.bytes[base_idx + 3],
        )
    }

    fn set_pixel(&mut self, x: u16, y: u16, color: Color) {
        let x = x as usize;
        let y = y as usize;
        let base_idx = y * 4 * self.width() as usize + x * 4;
        self.0.bytes[base_idx] = color.r;
        self.0.bytes[base_idx + 1] = color.g;
        self.0.bytes[base_idx + 2] = color.b;
        self.0.bytes[base_idx + 3] = color.a;
    }

    fn bytes(&self) -> &[u8] {
        &self.0.bytes
    }

    fn from_parts(width: u16, height: u16, bytes: &[u8]) -> Self {
        Self(Image {
            bytes: bytes.to_owned(),
            height,
            width,
        })
    }

    fn set_from(&mut self, other: Self) {
        let w = std::cmp::min(self.width(), other.width());
        let h = std::cmp::min(self.height(), other.height());

        for x in 0..w {
            for y in 0..h {
                self.set_pixel(x, y, other.pixel(x, y));
            }
        }
    }
}
