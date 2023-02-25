use crate::color::TRANSPARENT;
use crate::{graphics, AtomicAction, Bitmap, Color, FreeImage, Point, Position, Rect, Size};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum CanvasEffect {
    None,
    Update,
    New,
    Layer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Canvas<IMG> {
    inner: IMG,
}

impl<IMG: Bitmap> Canvas<IMG> {
    pub fn new(size: Size<i32>) -> Self {
        Self {
            inner: IMG::new(size, TRANSPARENT),
        }
    }

    pub fn inner(&self) -> &IMG {
        &self.inner
    }

    pub fn size(&self) -> Size<i32> {
        self.inner.size()
    }

    pub fn width(&self) -> i32 {
        self.inner.width()
    }

    pub fn height(&self) -> i32 {
        self.inner.height()
    }

    pub fn rect(&self) -> Rect<i32> {
        Rect::new(0, 0, self.width(), self.height())
    }

    pub fn bytes(&self) -> &[u8] {
        self.inner.bytes()
    }

    pub fn is_in_bounds(&self, p: Point<i32>) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.width() && p.y < self.height()
    }

    pub fn set_img(&mut self, img: IMG) {
        self.inner = img;
    }

    pub fn take_inner(&mut self) -> IMG {
        let old = std::mem::replace(&mut self.inner, IMG::new(Size::new(0, 0), TRANSPARENT));
        old
    }

    pub fn clear(&mut self) {
        self.inner = IMG::new(self.size(), TRANSPARENT);
    }

    pub fn resize(&mut self, size: Size<i32>) -> IMG {
        let new_img = IMG::new(size, TRANSPARENT);
        let old_img = std::mem::replace(&mut self.inner, new_img);
        self.inner.set_from(&old_img);

        old_img
    }

    pub fn pixel(&self, p: Point<i32>) -> Color {
        self.inner.pixel(p)
    }

    pub fn set_pixel(&mut self, p: Point<i32>, color: Color) -> Option<(Point<i32>, Color)> {
        if self.is_in_bounds(p) {
            let old = self.inner.pixel(p);

            if color == old {
                return None;
            }

            self.inner.set_pixel(p, color);
            return Some((p, old));
        }
        return None;
    }

    pub fn line(
        &mut self,
        p1: Point<i32>,
        p2: Point<i32>,
        color: Color,
    ) -> Vec<(Point<i32>, Color)> {
        let line = graphics::line(p1, p2);
        let mut reversals = Vec::new();

        for p in line {
            if let Some(action) = self.set_pixel(p, color) {
                reversals.push(action);
            }
        }
        reversals
    }

    pub fn rectangle(
        &mut self,
        p1: Point<i32>,
        p2: Point<i32>,
        color: Color,
    ) -> Vec<(Point<i32>, Color)> {
        let rect = graphics::rectangle(p1, p2);
        let mut reversals = Vec::new();

        for p in rect {
            if let Some(action) = self.set_pixel(p, color) {
                reversals.push(action);
            }
        }
        reversals
    }

    pub fn set_area(&mut self, area: Rect<i32>, color: Color) -> Vec<(Point<i32>, Color)> {
        let mut reversals = Vec::new();

        for i in 0..area.w {
            for j in 0..area.h {
                if let Some(action) = self.set_pixel((i + area.x, j + area.y).into(), color) {
                    reversals.push(action);
                }
            }
        }

        reversals
    }

    pub fn paste_obj(&mut self, obj: &FreeImage<IMG>) -> Vec<(Point<i32>, Color)> {
        let mut reversals = Vec::new();
        for i in 0..obj.rect.w {
            for j in 0..obj.rect.h {
                let ij = Point::new(i, j);
                let color = obj.texture.pixel(ij);
                let p = ij + obj.rect.pos();

                if self.is_in_bounds(p) {
                    let blended = color.blend_over(self.pixel(p));
                    if let Some(action) = self.set_pixel(p, blended) {
                        reversals.push(action);
                    }
                }
            }
        }

        reversals
    }

    pub fn bucket(&mut self, p: Point<i32>, color: Color) -> Vec<(Point<i32>, Color)> {
        let old_color = self.inner.pixel(p);

        if color == old_color {
            return Vec::new();
        }

        let w = self.inner.width() as usize;
        let h = self.inner.height() as usize;

        let mut marked = vec![false; w * h];
        let mut visit = vec![(p.x, p.y)];

        let mut reversals = Vec::new();

        loop {
            if visit.is_empty() {
                break;
            }

            let mut new_visit = Vec::new();
            while let Some((vx, vy)) = visit.pop() {
                marked[(vy as usize) * w + vx as usize] = true;

                if let Some(action) = self.set_pixel((vx, vy).into(), color) {
                    reversals.push(action);
                }

                for (nx, ny) in self.neighbors(vx, vy).into_iter().flatten() {
                    let ind = (ny as usize) * w + nx as usize;
                    if self.inner.pixel((nx, ny).into()) == old_color && !marked[ind] {
                        new_visit.push((nx, ny));
                        marked[ind] = true;
                    }
                }
            }

            visit.append(&mut new_visit);
        }

        reversals
    }

    fn neighbors(&self, x: i32, y: i32) -> [Option<(i32, i32)>; 4] {
        let mut neighbors = [None; 4];
        let w = self.inner.width();
        let h = self.inner.height();

        if x + 1 < w {
            neighbors[0] = Some((x + 1, y));
        }
        if x > 0 {
            neighbors[1] = Some((x - 1, y));
        }
        if y + 1 < h {
            neighbors[2] = Some((x, y + 1));
        }
        if y > 0 {
            neighbors[3] = Some((x, y - 1));
        }

        neighbors
    }

    pub fn img_from_area(&self, area: Rect<i32>) -> IMG {
        let mut img = IMG::new((area.w, area.h).into(), TRANSPARENT);

        for i in 0..area.w {
            for j in 0..area.h {
                let ij = Point::new(i, j);
                let p = area.pos() + ij;

                if p.x < self.width() && p.y < self.height() {
                    let color = self.pixel(p);
                    img.set_pixel(ij, color);
                }
            }
        }

        img
    }
}
