use crate::{Bitmap, Canvas, Size};

pub struct Layer<IMG: Bitmap> {
    canvas: Canvas<IMG>,
    visible: bool,
    opacity: u8,
}

impl<IMG: Bitmap> Layer<IMG> {
    pub fn new(size: Size<i32>) -> Self {
        Self {
            canvas: Canvas::new(size),
            visible: true,
            opacity: 255,
        }
    }

    pub fn canvas(&self) -> &Canvas<IMG> {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<IMG> {
        &mut self.canvas
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn opacity(&self) -> u8 {
        self.opacity
    }

    pub fn resize(&mut self, size: Size<i32>) {
        self.canvas.resize(size);
    }

    pub fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }
}
