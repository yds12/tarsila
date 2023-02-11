use crate::{Bitmap, Canvas};

pub struct Layer<IMG: Bitmap> {
    canvas: Canvas<IMG>,
    visible: bool,
    opacity: u8,
}

impl<IMG: Bitmap> Layer<IMG> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            canvas: Canvas::new(width, height),
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

    pub fn resize(&mut self, w: u16, h: u16) {
        self.canvas.resize(w, h);
    }

    pub fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }
}
