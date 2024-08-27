use crate::color::TRANSPARENT;
use crate::{graphics, Bitmap, Canvas, Color, Point, Position, Rect, Size};
use serde::{Deserialize, Serialize};

/// Represents an image that is not in any [`Canvas`], but floats freely on
/// the screen until it is *anchored* back into the canvas. Typical uses of this
/// are imported images and selections -- they can be moved around and
/// manipulated before being integrated into the canvas (by anchoring).
#[derive(Debug, Serialize, Deserialize)]
pub struct FreeImage<IMG> {
    pub rect: Rect<i32>,
    pub pivot: Option<Point<i32>>,
    pub texture: IMG,
}

impl<IMG: Bitmap> FreeImage<IMG> {
    /// Creates a new free image at a specified position, with predefined inner
    /// image
    pub fn new(p: Position<i32>, img: IMG) -> Self {
        Self {
            rect: Rect::new(p.x, p.y, img.width(), img.height()),
            texture: img,
            pivot: None,
        }
    }

    /// Creates a free image from the contents of the canvas at a specified
    /// area. A pivot point might be provided in case the image is being
    /// dragged/moved, in which case it would be the mouse position (relative to
    /// the image, (0, 0) being the top left corner of the image)
    pub fn from_canvas_area(
        canvas: &Canvas<IMG>,
        area: Rect<i32>,
        pivot: Option<Point<i32>>,
    ) -> Self {
        Self {
            rect: area,
            texture: canvas.img_from_area(area),
            pivot,
        }
    }

    // TODO: maybe we should have some helper to remove the offset of a set of
    // pixels, so that this function does not need to have this alien param.
    /// Create a free image from a set of pixels with a certain color. All other
    /// pixels will be set to transparent. If the pixels provided are offset by
    /// a certain amount, an offset can be passed so that they are discounted
    /// from the original pixel positions. If there is no offset, it should be
    /// (0, 0).
    pub fn from_pixels(
        size: Size<i32>,
        pixels: Vec<Point<i32>>,
        color: Color,
        offset: Position<i32>,
    ) -> Self {
        let mut img = IMG::new(size, TRANSPARENT);
        for point in pixels {
            img.set_pixel(point - offset, color);
        }

        Self::new(offset, img)
    }

    /// Creates a free image with a line between two points in a certain color.
    pub fn line_preview(p0: Point<i32>, p: Point<i32>, color: Color) -> Self {
        let span = p.abs_diff(p0);
        let offset = p.rect_min_corner(p0);

        FreeImage::from_pixels(span + Point::ONE, graphics::line(p0, p), color, offset)
    }

    /// Creates a free image with a rectangle between two points in a certain
    /// color.
    pub fn rect_preview(p0: Point<i32>, p: Point<i32>, color: Color) -> Self {
        let span = p.abs_diff(p0);
        let offset = p.rect_min_corner(p0);

        FreeImage::from_pixels(span + Point::ONE, graphics::rectangle(p0, p), color, offset)
    }

    /// Creates a free image with an ellipse between two points in a certain
    /// color.
    pub fn ellipse_preview(p0: Point<i32>, p: Point<i32>, color: Color) -> Self {
        let span = p.abs_diff(p0);
        let offset = p.rect_min_corner(p0);

        FreeImage::from_pixels(span + Point::ONE, graphics::ellipse(p0, p), color, offset)
    }

    /// Change the position of the free image considering that the passed point
    /// is the mouse position where it was released, and that the initial mouse
    /// position is defined by the pivot.
    pub fn move_by_pivot(&mut self, p: Point<i32>) {
        let pivot = self.pivot.unwrap_or((0, 0).into());
        let (dx, dy) = (p.x - pivot.x, p.y - pivot.y);
        self.rect.x = dx;
        self.rect.y = dy;
    }

    /// Flips the free image horizontally
    pub fn flip_horizontally(&mut self) {
        for i in 0..(self.rect.w / 2) {
            for j in 0..self.rect.h {
                let c1 = self.texture.pixel((i, j).into());
                let c2 = self.texture.pixel((self.rect.w - i - 1, j).into());
                self.texture.set_pixel((i, j).into(), c2);
                self.texture.set_pixel((self.rect.w - i - 1, j).into(), c1);
            }
        }
    }

    /// Flips the free image vertically
    pub fn flip_vertically(&mut self) {
        for j in 0..(self.rect.h / 2) {
            for i in 0..self.rect.w {
                let c1 = self.texture.pixel((i, j).into());
                let c2 = self.texture.pixel((i, self.rect.h - j - 1).into());
                self.texture.set_pixel((i, j).into(), c2);
                self.texture.set_pixel((i, self.rect.h - j - 1).into(), c1);
            }
        }
    }
}
