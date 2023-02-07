use crate::{Bitmap, Canvas, Rect};

pub struct FreeImage<IMG: Bitmap> {
    pub rect: Rect<i32>,
    pub texture: IMG,
}

impl<IMG: Bitmap> FreeImage<IMG> {
    pub fn new(x: i32, y: i32, img: IMG) -> Self {
        Self {
            rect: Rect::new(x, y, img.width() as i32, img.height() as i32),
            texture: img,
        }
    }

    pub fn from_canvas_area(canvas: &Canvas<IMG>, area: Rect<i32>) -> Self {
        Self {
            rect: area,
            texture: canvas.img_from_area(area),
        }
    }
}
