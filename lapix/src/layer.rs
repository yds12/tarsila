use crate::color::TRANSPARENT;
use crate::{Bitmap, Canvas, Color, Point, Rect, Size};

pub struct Layers<IMG: Bitmap> {
    inner: Vec<Layer<IMG>>,
    active: usize,
}

impl<IMG: Bitmap> Layers<IMG> {
    pub fn new(size: Size<i32>) -> Self {
        Self {
            inner: vec![Layer::new(size)],
            active: 0,
        }
    }

    pub fn active(&self) -> &Layer<IMG> {
        &self.inner[self.active]
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn count(&self) -> usize {
        self.inner.len()
    }

    pub fn canvas_at(&self, index: usize) -> &Canvas<IMG> {
        self.inner[index].canvas()
    }

    pub fn active_canvas(&self) -> &Canvas<IMG> {
        self.canvas_at(self.active)
    }

    pub fn get(&self, index: usize) -> &Layer<IMG> {
        &self.inner[index]
    }

    pub fn blended(&self) -> IMG {
        let w = self.canvas_at(0).width();
        let h = self.canvas_at(0).height();

        self.blended_area((0, 0, w, h).into())
    }

    pub fn blended_area(&self, r: Rect<i32>) -> IMG {
        let mut result = IMG::new((r.w, r.h).into(), TRANSPARENT);

        for i in 0..r.w {
            for j in 0..r.h {
                let ij = Point::new(i, j);
                result.set_pixel(ij, self.visible_pixel(ij + r.pos()));
            }
        }

        result
    }

    pub fn active_canvas_mut(&mut self) -> &mut Canvas<IMG> {
        self.inner[self.active].canvas_mut()
    }

    pub fn resize_all(&mut self, size: Size<i32>) {
        for layer in self.inner.iter_mut() {
            layer.resize(size);
        }
    }

    pub fn switch_to(&mut self, index: usize) {
        self.active = index;
    }

    pub fn add_above(&mut self) {
        let layer = Layer::new(self.active_canvas().size());
        self.inner.push(layer);
    }

    pub fn delete(&mut self, index: usize) {
        // TODO: this should not only remove it, as we need to be able to undo this
        self.inner.remove(index);
    }

    pub fn set_visibility(&mut self, index: usize, visible: bool) {
        self.inner[index].set_visibility(visible);
    }

    pub fn set_opacity(&mut self, index: usize, opacity: u8) {
        self.inner[index].set_opacity(opacity);
    }

    pub fn swap(&mut self, first: usize, second: usize) {
        self.inner.swap(first, second);
    }

    pub fn visible_pixel(&self, p: Point<i32>) -> Color {
        let mut result = if self.inner[0].visible() {
            self.canvas_at(0).pixel(p)
        } else {
            TRANSPARENT
        };

        for i in 1..self.count() {
            if !self.get(i).visible() {
                continue;
            }

            let color = self
                .canvas_at(i)
                .pixel(p)
                .with_multiplied_alpha(self.get(i).opacity());
            result = color.blend_over(result);
        }

        result
    }
}

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
