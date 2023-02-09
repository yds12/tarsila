use crate::{Bitmap, Canvas, Point, Rect};

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
