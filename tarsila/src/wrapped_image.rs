use lapix::{Bitmap, Color, Point, Size};
use macroquad::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct WrappedImage(pub Image);

impl<'a> Serialize for WrappedImage {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.into_png_bytes();
        ser.serialize_bytes(&bytes)
    }
}

impl<'a> Deserialize<'a> for WrappedImage {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        let vec = Vec::<u8>::deserialize(d).expect("failed to deserialize Vec<u8>");
        Ok(Self::from_file_bytes(vec))
    }
}

impl Bitmap for WrappedImage {
    fn new(size: Size<i32>, color: Color) -> Self {
        let bytes = vec![0; size.x as usize * size.y as usize * 4];
        let mut img = Self(Image {
            bytes,
            width: size.x as u16,
            height: size.y as u16,
        });

        for i in 0..size.x {
            for j in 0..size.y {
                img.set_pixel((i, j).into(), color);
            }
        }

        img
    }

    fn size(&self) -> Size<i32> {
        (self.0.width as i32, self.0.height as i32).into()
    }

    fn width(&self) -> i32 {
        self.0.width as i32
    }

    fn height(&self) -> i32 {
        self.0.height as i32
    }

    fn pixel(&self, p: Point<i32>) -> Color {
        let x = p.x as usize;
        let y = p.y as usize;

        let base_idx = y * 4 * self.width() as usize + x * 4;
        Color::new(
            self.0.bytes[base_idx],
            self.0.bytes[base_idx + 1],
            self.0.bytes[base_idx + 2],
            self.0.bytes[base_idx + 3],
        )
    }

    fn set_pixel(&mut self, p: Point<i32>, color: Color) {
        let x = p.x as usize;
        let y = p.y as usize;
        let base_idx = y * 4 * self.width() as usize + x * 4;
        self.0.bytes[base_idx] = color.r;
        self.0.bytes[base_idx + 1] = color.g;
        self.0.bytes[base_idx + 2] = color.b;
        self.0.bytes[base_idx + 3] = color.a;
    }

    fn bytes(&self) -> &[u8] {
        &self.0.bytes
    }

    fn from_parts(size: Size<i32>, bytes: &[u8]) -> Self {
        Self(Image {
            bytes: bytes.to_owned(),
            width: size.x as u16,
            height: size.y as u16,
        })
    }

    fn set_from(&mut self, other: &Self) {
        let w = std::cmp::min(self.width(), other.width());
        let h = std::cmp::min(self.height(), other.height());

        for x in 0..w {
            for y in 0..h {
                self.set_pixel((x, y).into(), other.pixel((x, y).into()));
            }
        }
    }
}
