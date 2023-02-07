use crate::{graphics, Bitmap, Color, FreeImage, Point, Position, Rect, Size};

#[derive(Debug, Clone, Copy)]
pub enum CanvasEffect {
    None,
    Update,
    New,
    Layer,
}

#[derive(Debug)]
pub enum CanvasAtomicEdit<IMG: Bitmap> {
    ChangePixel {
        position: Position<u16>,
        old: IMG::Color,
        new: IMG::Color,
    },
    ChangeSize {
        old: Size<u16>,
        new: Size<u16>,
    },
}

impl<IMG: Bitmap> CanvasAtomicEdit<IMG> {
    pub fn undo(&self) -> CanvasAtomicEdit<IMG> {
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
pub struct CanvasEdit<IMG: Bitmap>(Vec<CanvasAtomicEdit<IMG>>);

impl<IMG: Bitmap> CanvasEdit<IMG> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, edit: CanvasAtomicEdit<IMG>) {
        self.0.push(edit);
    }
    pub fn set_pixel(x: u16, y: u16, old: IMG::Color, new: IMG::Color) -> Self {
        Self(vec![CanvasAtomicEdit::ChangePixel {
            position: Position::new(x, y),
            old,
            new,
        }])
    }
    pub fn undo(&self) -> CanvasEdit<IMG> {
        let mut edits = Vec::new();
        for edit in &self.0 {
            edits.push(edit.undo());
        }

        Self(edits)
    }
}

pub struct Canvas<IMG: Bitmap> {
    inner: IMG,
    empty_color: IMG::Color,
    edits: Vec<CanvasEdit<IMG>>,
    cur_edit_bundle: Option<CanvasEdit<IMG>>,
}

impl<IMG: Bitmap> Canvas<IMG> {
    pub fn new(width: u16, height: u16) -> Self {
        let empty_color = IMG::Color::from_rgba(0, 0, 0, 0);
        Self {
            inner: IMG::new(width, height, empty_color),
            empty_color,
            edits: Vec::new(),
            cur_edit_bundle: None,
        }
    }

    pub fn inner(&self) -> &IMG {
        &self.inner
    }

    pub fn size(&self) -> Size<u16> {
        (self.width(), self.height()).into()
    }

    pub fn width(&self) -> u16 {
        self.inner.width()
    }

    pub fn height(&self) -> u16 {
        self.inner.height()
    }

    pub fn rect(&self) -> Rect<u16> {
        Rect::new(0, 0, self.width(), self.height())
    }

    pub fn bytes(&self) -> &[u8] {
        self.inner.bytes()
    }

    fn undo_edit(&mut self, edit: CanvasAtomicEdit<IMG>) -> CanvasEffect {
        // TODO: isn't this supposed to be in CanvasAtomicEdit?
        match edit {
            CanvasAtomicEdit::ChangePixel { position, old, .. } => {
                self.inner.set_pixel(position.x, position.y, old);

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

    fn register_set_pixel(&mut self, x: u16, y: u16, old: IMG::Color, new: IMG::Color) {
        if let Some(edit_bundle) = self.cur_edit_bundle.as_mut() {
            edit_bundle.push(CanvasAtomicEdit::ChangePixel {
                position: (x, y).into(),
                old,
                new,
            });
        }
    }

    pub fn clear(&mut self) {
        self.inner = IMG::new(self.width(), self.height(), self.empty_color);
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        let new_img = IMG::new(width, height, self.empty_color);
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

    pub fn pixel(&self, x: u16, y: u16) -> IMG::Color {
        self.inner.pixel(x, y)
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, color: IMG::Color) {
        if x < self.width() && y < self.height() {
            let old = self.inner.pixel(x, y);

            if color == old {
                return;
            }

            self.register_set_pixel(x, y, old, color);
            self.inner.set_pixel(x, y, color);
        }
    }

    pub fn line(&mut self, p1: Point<u16>, p2: Point<u16>, color: IMG::Color) {
        let line = graphics::line(p1, p2);

        for p in line {
            self.set_pixel(p.x, p.y, color);
        }
    }

    pub fn set_area(&mut self, area: Rect<u16>, color: IMG::Color) {
        for i in 0..area.w {
            for j in 0..area.h {
                self.set_pixel(i + area.x, j + area.y, color);
            }
        }
    }

    pub fn paste_obj(&mut self, obj: &FreeImage<IMG>) {
        self.start_editing_bundle();
        for i in 0..obj.rect.w {
            for j in 0..obj.rect.h {
                let color = obj.texture.pixel(i as u16, j as u16);
                let x = (i + obj.rect.x) as u16;
                let y = (j + obj.rect.y) as u16;
                let blended = color.blend_over(self.pixel(x, y));
                self.set_pixel(x, y, blended);
            }
        }
        self.finish_editing_bundle();
    }

    pub fn bucket(&mut self, x: u16, y: u16, color: IMG::Color) {
        let old_color = self.inner.pixel(x, y);

        if color == old_color {
            return;
        }

        let w = self.inner.width() as usize;
        let h = self.inner.height() as usize;

        let mut marked = vec![false; w * h];
        let mut visit = vec![(x, y)];

        loop {
            if visit.is_empty() {
                break;
            }

            let mut new_visit = Vec::new();
            while let Some((vx, vy)) = visit.pop() {
                marked[(vy as usize) * w + vx as usize] = true;
                self.set_pixel(vx, vy, color);

                for n in self.neighbors(vx, vy) {
                    if let Some((nx, ny)) = n {
                        let ind = (ny as usize) * w + nx as usize;
                        if self.inner.pixel(nx, ny) == old_color && !marked[ind] {
                            new_visit.push((nx, ny));
                            marked[ind] = true;
                        }
                    }
                }
            }

            visit.append(&mut new_visit);
        }
    }

    fn neighbors(&self, x: u16, y: u16) -> [Option<(u16, u16)>; 4] {
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
        let mut img = IMG::new(
            area.w as u16,
            area.h as u16,
            IMG::Color::from_rgba(0, 0, 0, 0),
        );

        for i in 0..area.w {
            for j in 0..area.h {
                let x = (area.x + i) as u16;
                let y = (area.y + j) as u16;

                if x < self.width() && y < self.height() {
                    let color = self.pixel(x, y);
                    img.set_pixel(i as u16, j as u16, color);
                }
            }
        }

        img
    }
}
