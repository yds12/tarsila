use crate::color::TRANSPARENT;
use crate::{Bitmap, Canvas, Color, Point, Position, Rect, Size};

pub struct FreeImage<IMG: Bitmap> {
    pub rect: Rect<i32>,
    pub pivot: Option<Point<i32>>,
    pub texture: IMG,
}

impl<IMG: Bitmap> FreeImage<IMG> {
    pub fn new(p: Position<i32>, img: IMG) -> Self {
        Self {
            rect: Rect::new(p.x, p.y, img.width() as i32, img.height() as i32),
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

    pub fn move_by_pivot(&mut self, p: Point<i32>) {
        let pivot = self.pivot.unwrap_or((0, 0).into());
        let (dx, dy) = (p.x - pivot.x as i32, p.y - pivot.y as i32);
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
