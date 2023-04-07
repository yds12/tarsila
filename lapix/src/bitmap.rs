use crate::{color, Color, Point, Size};

// TODO rename this trait
/// Represents a 2D matrix of pixels (an image)
pub trait Bitmap: Clone {
    /// Create a new image with a specified size and filled with one color
    fn new(size: Size<i32>, color: Color) -> Self;

    /// Get the [`Size`] of the image
    fn size(&self) -> Size<i32>;

    /// Get the width of the image in pixels
    fn width(&self) -> i32;

    /// Get the height of the image in pixels
    fn height(&self) -> i32;

    /// Get the pixel color at a certain [`Point`]
    fn pixel(&self, point: Point<i32>) -> Color;

    /// Set the pixel at a certain [`Point`] to a specified color
    fn set_pixel(&mut self, point: Point<i32>, color: Color);

    /// Get this image's representation as a slice of bytes
    fn bytes(&self) -> &[u8];

    /// Create a new image from a slice of bytes and a [`Size`]
    fn from_parts(size: Size<i32>, bytes: &[u8]) -> Self;

    /// Set this image pixels based on another, but respecting the size of this
    /// one, i.e. ignoring pixels out of bounds
    fn set_from(&mut self, other: &Self);

    // TODO: use this and next method to save/load files
    /// Get the the PNG representation of this image as a sequence of bytes
    fn png_bytes(&self) -> Vec<u8> {
        let img = image::RgbaImage::from_raw(
            self.width() as u32,
            self.height() as u32,
            self.bytes().to_owned(),
        )
        .expect("Failed to generate image from bytes");

        let vec = Vec::<u8>::new();
        let mut vec = std::io::Cursor::new(vec);
        img.write_to(&mut vec, image::ImageOutputFormat::Png)
            .unwrap(); // never fails

        vec.into_inner()
    }

    /// Create a new image from a sequence of bytes read from an image file
    /// (e.g. PNG or JPG)
    fn from_file_bytes(bytes: Vec<u8>) -> Self {
        let reader = image::io::Reader::new(std::io::Cursor::new(bytes))
            .with_guessed_format()
            .unwrap(); // never fails
        let img = reader.decode().expect("failed to decode image");
        let img = img.into_rgba8();

        let size: Size<i32> = (img.width() as i32, img.height() as i32).into();
        let mut bitmap = Self::new(size, color::TRANSPARENT);

        for (x, y, pixel) in img.enumerate_pixels() {
            let color = Color::new(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
            bitmap.set_pixel((x as i32, y as i32).into(), color);
        }

        bitmap
    }
}

#[cfg(feature = "test-utils")]
pub use test::TestImage;

#[cfg(feature = "test-utils")]
mod test {
    use super::*;
    use serde::{Deserialize, Serialize};

    // TODO implementing this showed that our trait is not that generic. We were
    // forced to add this `bytes` field which defeats the purpose of having a
    // vec of vec of color as storage. We should improve this.
    /// Terribly slow but easy to use image type just for tests' sake
    #[derive(Clone, Serialize, Deserialize)]
    pub struct TestImage {
        size: Size<i32>,
        pixels: Vec<Vec<Color>>, // vec of rows of pixels
        bytes: Vec<u8>,
    }

    impl TestImage {
        fn update_bytes(&mut self) {
            self.bytes = self
                .pixels
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|c| [c.r, c.g, c.b, c.a])
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
        }
    }

    impl Bitmap for TestImage {
        fn new(size: Size<i32>, color: Color) -> Self {
            let pixels = vec![vec![color; size.x as usize]; size.y as usize];

            let mut img = Self {
                size,
                pixels,
                bytes: Vec::new(),
            };
            img.update_bytes();

            img
        }
        fn size(&self) -> Size<i32> {
            self.size
        }
        fn width(&self) -> i32 {
            self.size.x
        }
        fn height(&self) -> i32 {
            self.size.y
        }
        fn pixel(&self, point: Point<i32>) -> Color {
            self.pixels[point.y as usize][point.x as usize]
        }
        fn set_pixel(&mut self, point: Point<i32>, color: Color) {
            self.pixels[point.y as usize][point.x as usize] = color;
            self.update_bytes();
        }
        fn bytes(&self) -> &[u8] {
            &self.bytes
        }
        fn from_parts(size: Size<i32>, bytes: &[u8]) -> Self {
            todo!()
        }
        fn set_from(&mut self, other: &Self) {
            self.pixels = other.pixels.clone();

            if self.pixels.len() >= self.size.y as usize {
                self.pixels = self.pixels[0..self.size.y as usize].to_vec();
            } else {
                while self.pixels.len() < self.size.y as usize {
                    self.pixels
                        .push(vec![color::TRANSPARENT; self.size.x as usize]);
                }
            }

            if self.pixels[0].len() >= self.size.x as usize {
                for row in self.pixels.iter_mut() {
                    *row = row[0..self.size.x as usize].to_vec();
                }
            } else {
                for row in self.pixels.iter_mut() {
                    while row.len() < self.size.x as usize {
                        row.push(color::TRANSPARENT);
                    }
                }
            }
        }
    }
}
