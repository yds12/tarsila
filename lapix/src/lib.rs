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
pub use layer::Layer;
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
    fn new(width: u16, height: u16, color: Color) -> Self;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn pixel(&self, x: u16, y: u16) -> Color;
    fn set_pixel(&mut self, x: u16, y: u16, color: Color);
    fn bytes(&self) -> &[u8];
    fn from_parts(width: u16, height: u16, bytes: &[u8]) -> Self;
    fn set_from(&mut self, other: Self);
}
