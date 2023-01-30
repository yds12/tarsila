use lapix_core::Bitmap;
use macroquad::prelude::*;

pub struct WrappedImage(pub Image);

impl Bitmap for WrappedImage {
    type Color = [u8; 4];

    fn new(width: u16, height: u16, color: Self::Color) -> Self {
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
    fn pixel(&self, x: u16, y: u16) -> Self::Color {
        let base_idx = y * 4 * self.width() + x * 4;
        [
            self.0.bytes[base_idx as usize],
            self.0.bytes[(base_idx + 1) as usize],
            self.0.bytes[(base_idx + 2) as usize],
            self.0.bytes[(base_idx + 3) as usize],
        ]
    }
    fn set_pixel(&mut self, x: u16, y: u16, color: Self::Color) {
        let base_idx = y * 4 * self.width() + x * 4;
        self.0.bytes[base_idx as usize] = color[0];
        self.0.bytes[(base_idx + 1) as usize] = color[1];
        self.0.bytes[(base_idx + 2) as usize] = color[2];
        self.0.bytes[(base_idx + 3) as usize] = color[3];
    }
    fn bytes(&self) -> &[u8] {
        &self.0.bytes
    }
}

