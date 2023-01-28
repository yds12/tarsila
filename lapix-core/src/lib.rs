mod color;

pub use color::Color;

pub trait Bitmap {
    type Color: Color;

    fn new(width: u16, height: u16, color: Self::Color) -> Self;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn pixel(&self, x: u16, y: u16) -> Self::Color;
    fn set_pixel(&mut self, x: u16, y: u16, color: Self::Color);
}

/*
pub struct Canvas<IMG: Bitmap> {
    inner: IMG
}

impl<IMG: Bitmap> Canvas<IMG> {
    fn new(width: u16, height: u16) -> Self {
        Self {
            inner: IMG::new(width, height, color::WHITE)
        }
    }
}
*/
