use crate::color::TRANSPARENT;
use crate::{graphics, Bitmap, Color, FreeImage, Point, Position, Rect, Size};

#[derive(Debug, Clone, Copy)]
pub enum CanvasEffect {
    None,
    Update,
    New,
    Layer,
}

#[derive(Debug)]
pub enum CanvasAtomicEdit {
    ChangePixel {
        position: Position<i32>,
        old: Color,
        new: Color,
    },
    ChangeSize {
        old: Size<i32>,
        new: Size<i32>,
    },
}

impl CanvasAtomicEdit {
    pub fn undo(&self) -> CanvasAtomicEdit {
        match self {
            CanvasAtomicEdit::ChangePixel { position, old, new } => CanvasAtomicEdit::ChangePixel {
                position: *position,
                old: *new,
                new: *old,
            },
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct CanvasEdit(Vec<CanvasAtomicEdit>);

impl CanvasEdit {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, edit: CanvasAtomicEdit) {
        self.0.push(edit);
    }
    pub fn set_pixel(position: Position<i32>, old: Color, new: Color) -> Self {
        Self(vec![CanvasAtomicEdit::ChangePixel { position, old, new }])
    }
    pub fn undo(&self) -> CanvasEdit {
        let mut edits = Vec::new();
        for edit in &self.0 {
            edits.push(edit.undo());
        }

        Self(edits)
    }
}

pub struct Canvas<IMG: Bitmap> {
    inner: IMG,
    edits: Vec<CanvasEdit>,
    cur_edit_bundle: Option<CanvasEdit>,
}

impl<IMG: Bitmap> Canvas<IMG> {
    pub fn new(size: Size<i32>) -> Self {
        Self {
            inner: IMG::new(size, TRANSPARENT),
            edits: Vec::new(),
            cur_edit_bundle: None,
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

    fn undo_edit(&mut self, edit: CanvasAtomicEdit) -> CanvasEffect {
        // TODO: isn't this supposed to be in CanvasAtomicEdit?
        match edit {
            CanvasAtomicEdit::ChangePixel { position, old, .. } => {
                self.inner.set_pixel(position, old);

                CanvasEffect::Update
            }
            _ => todo!(),
        }
    }

    pub fn undo_last(&mut self) -> CanvasEffect {
        // TODO: here we just try undo the last, but we need to keep popping
        // until a undo-relevant event is found (e.g. we need to remove UNDO
        // events from the stack)
        let edit = self.edits.pop();
        let mut effect = CanvasEffect::None;

        if let Some(edit) = edit {
            for atomic_edit in edit.0 {
                effect = self.undo_edit(atomic_edit);
            }
        }

        effect
    }

    fn register_set_pixel(&mut self, position: Position<i32>, old: Color, new: Color) {
        if let Some(edit_bundle) = self.cur_edit_bundle.as_mut() {
            edit_bundle.push(CanvasAtomicEdit::ChangePixel { position, old, new });
        }
    }

    pub fn clear(&mut self) {
        self.inner = IMG::new(self.size(), TRANSPARENT);
    }

    pub fn resize(&mut self, size: Size<i32>) {
        let new_img = IMG::new(size, TRANSPARENT);
        let old_img = std::mem::replace(&mut self.inner, new_img);
        self.inner.set_from(old_img);
    }

    pub fn start_editing_bundle(&mut self) {
        self.cur_edit_bundle = Some(CanvasEdit::new());
    }

    pub fn finish_editing_bundle(&mut self) {
        if let Some(edit_bundle) = self.cur_edit_bundle.take() {
            self.edits.push(edit_bundle);
        } else {
            eprintln!("WARN: Trying to finish tool action when there's no edit bundle.");
        }
    }

    pub fn pixel(&self, p: Point<i32>) -> Color {
        self.inner.pixel(p)
    }

    pub fn set_pixel(&mut self, p: Point<i32>, color: Color) {
        if self.is_in_bounds(p) {
            let old = self.inner.pixel(p);

            if color == old {
                return;
            }

            self.register_set_pixel(p, old, color);
            self.inner.set_pixel(p, color);
        }
    }

    pub fn line(&mut self, p1: Point<i32>, p2: Point<i32>, color: Color) {
        let line = graphics::line(p1, p2);

        for p in line {
            self.set_pixel(p, color);
        }
    }

    pub fn rectangle(&mut self, p1: Point<i32>, p2: Point<i32>, color: Color) {
        let rect = graphics::rectangle(p1, p2);

        for p in rect {
            self.set_pixel(p, color);
        }
    }

    pub fn set_area(&mut self, area: Rect<i32>, color: Color) {
        for i in 0..area.w {
            for j in 0..area.h {
                self.set_pixel((i + area.x, j + area.y).into(), color);
            }
        }
    }

    pub fn paste_obj(&mut self, obj: &FreeImage<IMG>) {
        self.start_editing_bundle();
        for i in 0..obj.rect.w {
            for j in 0..obj.rect.h {
                let ij = Point::new(i, j);
                let color = obj.texture.pixel(ij);
                let p = ij + obj.rect.pos();

                if self.is_in_bounds(p) {
                    let blended = color.blend_over(self.pixel(p));
                    self.set_pixel(p, blended);
                }
            }
        }
        self.finish_editing_bundle();
    }

    pub fn bucket(&mut self, p: Point<i32>, color: Color) {
        let old_color = self.inner.pixel(p);

        if color == old_color {
            return;
        }

        let w = self.inner.width() as usize;
        let h = self.inner.height() as usize;

        let mut marked = vec![false; w * h];
        let mut visit = vec![(p.x, p.y)];

        loop {
            if visit.is_empty() {
                break;
            }

            let mut new_visit = Vec::new();
            while let Some((vx, vy)) = visit.pop() {
                marked[(vy as usize) * w + vx as usize] = true;
                self.set_pixel((vx, vy).into(), color);

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
