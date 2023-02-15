use std::fmt::Debug;

mod canvas;
mod color;
mod event;
mod floating;
pub mod graphics;
mod layer;
mod palette;
pub mod primitives;
mod state;
mod util;

pub use canvas::{Canvas, CanvasEffect};
pub use color::Color;
pub use event::Event;
pub use floating::FreeImage;
pub use layer::{Layer, Layers};
use palette::Palette;
pub use primitives::*;
pub use state::{Selection, State};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Tool {
    Brush,
    Eraser,
    Eyedropper,
    Bucket,
    Line,
    Selection,
    Move,
    Rectangle,
}

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
}
