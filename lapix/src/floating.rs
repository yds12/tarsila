use crate::color::TRANSPARENT;
use crate::{Bitmap, Canvas, Color, Point, Position, Rect, Size};

pub struct FreeImage<IMG: Bitmap> {
    pub rect: Rect<i32>,
    pub pivot: Option<Point<u16>>,
    pub texture: IMG,
}

impl<IMG: Bitmap> FreeImage<IMG> {
    pub fn new(x: i32, y: i32, img: IMG) -> Self {
        Self {
            rect: Rect::new(x, y, img.width() as i32, img.height() as i32),
            texture: img,
            pivot: None,
        }
    }

    pub fn from_canvas_area(
        canvas: &Canvas<IMG>,
        area: Rect<i32>,
        pivot: Option<Point<u16>>,
    ) -> Self {
        Self {
            rect: area,
            texture: canvas.img_from_area(area),
            pivot,
        }
    }

    pub fn from_pixels(
        size: Size<u16>,
        pixels: Vec<Point<i32>>,
        color: Color,
        offset: Position<i32>,
    ) -> Self {
        let mut img = IMG::new(size.x, size.y, TRANSPARENT);
        for point in pixels {
            let x = (point.x - offset.x) as u16;
            let y = (point.y - offset.y) as u16;
            img.set_pixel(x, y, color);
        }

        Self::new(offset.x, offset.y, img)
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
                let c1 = self.texture.pixel(i as u16, j as u16);
                let c2 = self.texture.pixel((self.rect.w - i - 1) as u16, j as u16);
                self.texture.set_pixel(i as u16, j as u16, c2);
                self.texture
                    .set_pixel((self.rect.w - i - 1) as u16, j as u16, c1);
            }
        }
    }

    pub fn flip_vertically(&mut self) {
        for j in 0..(self.rect.h / 2) {
            for i in 0..self.rect.w {
                let c1 = self.texture.pixel(i as u16, j as u16);
                let c2 = self.texture.pixel(i as u16, (self.rect.h - j - 1) as u16);
                self.texture.set_pixel(i as u16, j as u16, c2);
                self.texture
                    .set_pixel(i as u16, (self.rect.h - j - 1) as u16, c1);
            }
        }
    }
}
