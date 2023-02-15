use crate::color::TRANSPARENT;
use crate::{graphics, Bitmap, Canvas, Color, Point, Position, Rect, Size};

pub struct FreeImage<IMG: Bitmap> {
    pub rect: Rect<i32>,
    pub pivot: Option<Point<i32>>,
    pub texture: IMG,
}

impl<IMG: Bitmap> FreeImage<IMG> {
    pub fn new(p: Position<i32>, img: IMG) -> Self {
        Self {
            rect: Rect::new(p.x, p.y, img.width(), img.height()),
            texture: img,
            pivot: None,
        }
    }

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

    pub fn line_preview(p0: Point<i32>, p: Point<i32>, color: Color) -> Option<Self> {
        let span = p.abs_diff(p0);
        let offset = p.rect_min_corner(p0);

        Some(FreeImage::from_pixels(
            span + Point::ONE,
            graphics::line(p0, p),
            color,
            offset,
        ))
    }

    pub fn rect_preview(p0: Point<i32>, p: Point<i32>, color: Color) -> Option<Self> {
        let span = p.abs_diff(p0);
        let offset = p.rect_min_corner(p0);

        Some(FreeImage::from_pixels(
            span + Point::ONE,
            graphics::rectangle(p0, p),
            color,
            offset,
        ))
    }

    pub fn move_by_pivot(&mut self, p: Point<i32>) {
        let pivot = self.pivot.unwrap_or((0, 0).into());
        let (dx, dy) = (p.x - pivot.x, p.y - pivot.y);
        self.rect.x = dx;
        self.rect.y = dy;
    }

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
