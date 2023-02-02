use std::fmt::Debug;

mod color;
mod event;
mod graphics;
pub mod primitives;

pub use color::Color;
pub use event::Event;
pub use primitives::*;

#[derive(Debug, Clone, Copy)]
pub enum CanvasEffect {
    None,
    Update,
    New,
    Layer
}

pub struct State<IMG: Bitmap> {
    layers: Vec<Canvas<IMG>>,
    active_layer: usize,
    events: Vec<Event<IMG>>,
    tool: Tool,
    main_color: IMG::Color,
}

impl<IMG: Bitmap + Debug> State<IMG> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            layers: vec![Canvas::new(width, height)],
            active_layer: 0,
            events: Vec::new(),
            tool: Tool::Brush,
            main_color: IMG::Color::from_rgb(0, 0, 0),
        }
    }

    pub fn execute(&mut self, event: Event<IMG>) -> CanvasEffect {
        if Some(&event) == self.events.last() && !event.repeatable() {
            return CanvasEffect::None;
        }

        dbg!(&event);
        let t0 = std::time::SystemTime::now();
        match event.clone() {
            Event::ClearCanvas => self.canvas_mut().clear(),
            Event::ResizeCanvas(w, h) => self.canvas_mut().resize(w, h),
            Event::BrushStart | Event::LineStart(_, _) | Event::EraseStart => {
                self.canvas_mut().start_tool_action()
            }
            Event::BrushEnd | Event::EraseEnd => self.canvas_mut().finish_tool_action(),
            Event::LineEnd(x, y) => {
                let last_event = self.events.last();
                let point = match last_event {
                    Some(Event::LineStart(i, j)) => (*i, *j).into(),
                    _ => panic!("line not started!"),
                };
                let color = self.main_color.clone();
                self.canvas_mut().line(point, (x, y).into(), color);
                self.canvas_mut().finish_tool_action();
            }
            Event::BrushStroke(x, y) => {
                let last_event = self.events.last();
                if let Some(Event::BrushStroke(x0, y0)) = last_event {
                    let color = self.main_color.clone();
                    let p0 = (*x0, *y0).into();
                    self.canvas_mut().line(p0, (x, y).into(), color);
                } else {
                    let color = self.main_color.clone();
                    self.canvas_mut().set_pixel(x, y, color);
                }
            }
            Event::SetTool(tool) => self.tool = tool,
            Event::SetMainColor(color) => self.main_color = color,
            Event::Save(path) => self.save_image(path.to_string_lossy().as_ref()),
            Event::OpenFile(path) => self.open_image(path.to_string_lossy().as_ref()),
            Event::Bucket(x, y) => {
                self.canvas_mut().start_tool_action();
                let color = self.main_color.clone();
                let effect = self.canvas_mut().bucket(x, y, color);
                self.canvas_mut().finish_tool_action();

                effect
            }
            Event::Erase(x, y) => {
                if self.canvas_mut().inner.pixel(x, y) != IMG::Color::from_rgba(0, 0, 0, 0) {
                    self.canvas_mut()
                        .set_pixel(x, y, IMG::Color::from_rgba(0, 0, 0, 0))
                }
            }
            Event::NewLayerAbove => self.add_layer(),
            Event::NewLayerBelow => todo!(),
            Event::SwitchLayer(i) => self.active_layer = i,
            Event::Undo => {
                // TODO: we should add UNDO to the events list
                dbg!(t0.elapsed().unwrap());
                return self.undo();
            }
            _ => todo!(),
        }
        dbg!(t0.elapsed().unwrap());

        let effect = event.canvas_effect();
        self.events.push(event);

        effect
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<IMG> {
        &mut self.layers[self.active_layer]
    }

    pub fn canvas(&self) -> &Canvas<IMG> {
        &self.layers[self.active_layer]
    }

    pub fn layer(&self, index: usize) -> &Canvas<IMG> {
        &self.layers[index]
    }

    pub fn selected_tool(&self) -> Tool {
        self.tool
    }

    pub fn main_color(&self) -> IMG::Color {
        self.main_color
    }

    pub fn active_layer(&self) -> usize {
        self.active_layer
    }

    pub fn num_layers(&self) -> usize {
        self.layers.len()
    }

    fn add_layer(&mut self) {
        let layer = Canvas::<IMG>::new(self.canvas().width(), self.canvas().height());
        self.layers.push(layer);
    }

    fn save_image(&self, path: &str) {
        let bytes = self.canvas().inner.bytes();
        let img = image::RgbaImage::from_raw(
            self.canvas().width() as u32,
            self.canvas().height() as u32,
            bytes.to_owned(),
        )
        .expect("Failed to generate image from bytes");
        img.save(path).expect("Failed to save image");
    }

    fn open_image(&mut self, path: &str) {
        use image::io::Reader as ImageReader;
        use std::io::Cursor;

        let img = ImageReader::open(path).unwrap().decode().unwrap();
        let img = img.into_rgba8();
        self.canvas_mut().resize(img.width() as u16, img.height() as u16);

        for (x, y, pixel) in img.enumerate_pixels() {
            let color = IMG::Color::from_rgba(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
            self.canvas_mut().set_pixel(x as u16, y as u16, color);
        }
    }

    fn undo(&mut self) -> CanvasEffect {
        self.canvas_mut().undo_last()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Tool {
    Brush,
    Eraser,
    Eyedropper,
    Bucket,
    Line,
}

pub trait Bitmap {
    type Color: Color;

    fn new(width: u16, height: u16, color: Self::Color) -> Self;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn pixel(&self, x: u16, y: u16) -> Self::Color;
    fn set_pixel(&mut self, x: u16, y: u16, color: Self::Color);
    fn bytes(&self) -> &[u8];
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
    fn new(width: u16, height: u16) -> Self {
        let empty_color = IMG::Color::from_rgba(0, 0, 0, 0);
        Self {
            inner: IMG::new(width, height, empty_color),
            empty_color,
            edits: Vec::new(),
            cur_edit_bundle: None,
        }
    }
    fn undo_edit(&mut self, edit: CanvasAtomicEdit<IMG>) -> CanvasEffect {
        match edit {
            CanvasAtomicEdit::ChangePixel { position, old, .. } => {
                self.inner.set_pixel(position.x, position.y, old);

                CanvasEffect::Update
            }
            _ => todo!(),
        }
    }
    fn undo_last(&mut self) -> CanvasEffect {
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
    fn clear(&mut self) {
        self.inner = IMG::new(self.width(), self.height(), self.empty_color);
    }
    fn resize(&mut self, width: u16, height: u16) {
        // TODO: it's clearing the image, but it shouldn't
        self.inner = IMG::new(width, height, self.empty_color);
    }
    fn start_tool_action(&mut self) {
        self.cur_edit_bundle = Some(CanvasEdit::new());
    }
    fn finish_tool_action(&mut self) {
        if let Some(edit_bundle) = self.cur_edit_bundle.take() {
            self.edits.push(edit_bundle);
        } else {
            eprintln!("WARN: Trying to finish tool action when there's no edit bundle.");
        }
    }
    fn set_pixel(&mut self, x: u16, y: u16, color: IMG::Color) {
        let old = self.inner.pixel(x, y);

        if color == old {
            return;
        }

        self.register_set_pixel(x, y, old, color);
        self.inner.set_pixel(x, y, color);
    }
    fn line(&mut self, p1: Point<u16>, p2: Point<u16>, color: IMG::Color) {
        let line = graphics::line(p1, p2);

        for p in line {
            self.set_pixel(p.x, p.y, color);
        }
    }
    fn bucket(&mut self, x: u16, y: u16, color: IMG::Color) {
        //self.cur_edit_bundle = Some(CanvasEdit::new());
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
        //self.edits.push(self.cur_edit_bundle.take().unwrap());
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
    pub fn inner(&self) -> &IMG {
        &self.inner
    }
    pub fn width(&self) -> u16 {
        self.inner.width()
    }
    pub fn height(&self) -> u16 {
        self.inner.height()
    }
}
