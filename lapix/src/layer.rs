use crate::color::TRANSPARENT;
use crate::{Bitmap, Canvas, Color, Point, Rect, Size};
use serde::{Deserialize, Serialize};

/// An ordered collection of [`Layer`]s. There is always one active layer.
#[derive(Debug, Serialize, Deserialize)]
pub struct Layers<IMG> {
    inner: Vec<Layer<IMG>>,
    active: usize,
}

impl<IMG: Bitmap> Layers<IMG> {
    /// Creates a new set of layers
    pub fn new(size: Size<i32>) -> Self {
        Self {
            inner: vec![Layer::new(size)],
            active: 0,
        }
    }

    /// Get the active [`Layer`]
    pub fn active(&self) -> &Layer<IMG> {
        &self.inner[self.active]
    }

    /// Get the index of the active [`Layer`]
    pub fn active_index(&self) -> usize {
        self.active
    }

    /// Get the number of [`Layer`]s
    pub fn count(&self) -> usize {
        self.inner.len()
    }

    /// Get the [`Canvas`] of the [`Layer`] at the specified index
    pub fn canvas_at(&self, index: usize) -> &Canvas<IMG> {
        self.inner[index].canvas()
    }

    /// Get the [`Canvas`] of the active [`Layer`]
    pub fn active_canvas(&self) -> &Canvas<IMG> {
        self.canvas_at(self.active)
    }

    /// Get a [`Layer`] by its index
    pub fn get(&self, index: usize) -> &Layer<IMG> {
        &self.inner[index]
    }

    /// Get an image of all the [`Layer`]s blended together
    pub fn blended(&self) -> IMG {
        let w = self.canvas_at(0).width();
        let h = self.canvas_at(0).height();

        self.blended_area((0, 0, w, h).into())
    }

    /// Get an image of an area (determined by a rectangle) of all [`Layer`]s
    /// blended together
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

    /// Get a mutable reference to a [`Layer`] by its index
    pub fn get_mut(&mut self, index: usize) -> &mut Layer<IMG> {
        &mut self.inner[index]
    }

    /// Get a mutable reference to the [`Canvas`] of the [`Layer`] at a certain
    /// index
    pub fn canvas_at_mut(&mut self, index: usize) -> &mut Canvas<IMG> {
        self.inner[index].canvas_mut()
    }

    /// Get a mutable reference to the [`Canvas`] of the active [`Layer`]
    pub fn active_canvas_mut(&mut self) -> &mut Canvas<IMG> {
        self.inner[self.active].canvas_mut()
    }

    /// Resize all [`Layer`]s, returning the images that were there before the
    /// resizing (used for undoing)
    pub fn resize_all(&mut self, size: Size<i32>) -> Vec<IMG> {
        let mut imgs = Vec::new();
        for layer in self.inner.iter_mut() {
            let img = layer.resize(size);
            imgs.push(img);
        }

        imgs
    }

    /// Set the active [`Layer`] to the specified index
    pub fn switch_to(&mut self, index: usize) {
        self.active = index;
    }

    /// Add a new [`Layer`] above all layers
    pub fn add_new_above(&mut self) {
        let layer = Layer::new(self.active_canvas().size());
        self.inner.push(layer);
    }

    /// Add a new [`Layer`] at the specified index
    pub fn add_at(&mut self, index: usize, layer: Layer<IMG>) {
        self.inner.insert(index, layer);
    }

    /// Delete the [`Layer`] at the specified index
    pub fn delete(&mut self, index: usize) -> Layer<IMG> {
        let layer = self.inner.remove(index);
        self.active = self.active.clamp(0, self.count() - 1);

        layer
    }

    /// Set whether the [`Layer`] at the specified index is visible or not
    pub fn set_visibility(&mut self, index: usize, visible: bool) {
        self.inner[index].set_visibility(visible);
    }

    /// Set the opacity (alpha) of the [`Layer`] at the specified index
    pub fn set_opacity(&mut self, index: usize, opacity: u8) {
        self.inner[index].set_opacity(opacity);
    }

    /// Swap the positions of two [`Layer`]s
    pub fn swap(&mut self, first: usize, second: usize) {
        self.inner.swap(first, second);
    }

    // TODO: maybe Canvas is a better name for Layers than for that type, since
    // the canvas is a combination of all layers, not a single layer's image
    /// Get the color of the visible pixel at a certain [`Point`] in the canvas,
    /// considering the blended result of all layers with their visibility and
    /// opacity settings
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

/// Represents a layer of the canvas. Layers are stacked on top of each other to
/// make a final image, blending colors with transparency. Layers can be moved
/// up or down relative to each other, can be made invisible or have a level of
/// transparency (opacity).
#[derive(Debug, Serialize, Deserialize)]
pub struct Layer<IMG> {
    canvas: Canvas<IMG>,
    visible: bool,
    opacity: u8,
}

impl<IMG: Bitmap> Layer<IMG> {
    /// Create a new layer with a specified size
    pub fn new(size: Size<i32>) -> Self {
        Self {
            canvas: Canvas::new(size),
            visible: true,
            opacity: 255,
        }
    }

    /// Get the [`Canvas`] of this layer
    pub fn canvas(&self) -> &Canvas<IMG> {
        &self.canvas
    }

    /// Get a mutable reference to the [`Canvas`] of this layer
    pub fn canvas_mut(&mut self) -> &mut Canvas<IMG> {
        &mut self.canvas
    }

    /// Whether this layer is visible
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Get the opacity level (alpha) of this layer, a value from 0-255
    pub fn opacity(&self) -> u8 {
        self.opacity
    }

    /// Take the image of this layer's [`Canvas`], leaving a dummy empty one in
    /// its place
    pub fn take_img(&mut self) -> IMG {
        self.canvas.take_inner()
    }

    /// Resize this layer, returning the previous image (the image before the
    /// resizing)
    pub fn resize(&mut self, size: Size<i32>) -> IMG {
        self.canvas.resize(size)
    }

    /// Set whether this layer is visible
    pub fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set the opacity of this layer
    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }
}
