mod action;
mod canvas;
mod color;
mod event;
mod floating;
pub mod graphics;
mod layer;
mod palette;
pub mod primitives;
mod state;
mod tool;
mod transform;
mod util;

use action::{Action, AtomicAction};
pub use canvas::{Canvas, CanvasEffect};
pub use color::Color;
pub use event::Event;
pub use floating::FreeImage;
pub use layer::{Layer, Layers};
use palette::Palette;
pub use primitives::*;
use serde::{Deserialize, Serialize};
pub use state::{Selection, State};
use std::fmt::Debug;
pub use tool::Tool;
use transform::Transform;

// TODO: move this to own file
pub trait Bitmap: Clone {
    fn new(size: Size<i32>, color: Color) -> Self;
    fn size(&self) -> Size<i32>;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn pixel(&self, point: Point<i32>) -> Color;
    fn set_pixel(&mut self, point: Point<i32>, color: Color);
    fn bytes(&self) -> &[u8];
    fn from_parts(size: Size<i32>, bytes: &[u8]) -> Self;
    fn set_from(&mut self, other: Self);
    // TODO: use this and next methods to save/load files
    fn into_png_bytes(&self) -> Vec<u8> {
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
