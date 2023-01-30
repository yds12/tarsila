use std::path::PathBuf;
use std::fmt::Debug;

mod color;

pub use color::Color;

pub struct State<IMG: Bitmap> {
    canvas: Canvas<IMG>,
    tool: Tool,
    main_color: IMG::Color,
}

impl<IMG: Bitmap + Debug> State<IMG> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            canvas: Canvas::new(width, height),
            tool: Tool::Brush,
            main_color: IMG::Color::from_rgb(0, 0, 0),
        }
    }
    pub fn execute(&mut self, event: Event<IMG>) {
        match event {
            Event::ClearCanvas => self.canvas.clear(),
            Event::ResizeCanvas(w, h) => self.canvas.resize(w, h),
            Event::BrushOnPixel(x, y) => self.canvas.set_pixel(x, y, self.main_color),
            Event::SetTool(tool) => self.tool = tool,
            Event::SetMainColor(color) => self.main_color = color,
            Event::Save(path) => self.save_image(path.to_string_lossy().as_ref()),
            Event::Bucket(x, y) => self.canvas.bucket(x, y, self.main_color),
            Event::Erase(x, y) => self.canvas.set_pixel(x, y, IMG::Color::from_rgba(0, 0, 0, 0))
        }
    }
    pub fn canvas(&self) -> &Canvas<IMG> {
        &self.canvas
    }
    pub fn selected_tool(&self) -> Tool {
        self.tool
    }
    pub fn main_color(&self) -> IMG::Color {
        self.main_color
    }
    fn save_image(&self, path: &str) {
        let bytes = self.canvas.inner.bytes();
        let img = image::RgbaImage::from_raw(
            self.canvas.width() as u32,
            self.canvas.height() as u32,
            bytes.to_owned(),
        )
        .expect("Failed to generate image from bytes");
        img.save(path).expect("Failed to save image");
    }
}

#[derive(Debug, Clone)]
pub enum Event<IMG: Bitmap> {
    ClearCanvas,
    ResizeCanvas(u16, u16),
    BrushOnPixel(u16, u16),
    SetTool(Tool),
    SetMainColor(IMG::Color),
    Save(PathBuf),
    Bucket(u16, u16),
    Erase(u16, u16)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tool {
    Brush,
    Eraser,
    Eyedropper,
    Bucket
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

pub struct Canvas<IMG: Bitmap> {
    inner: IMG,
    empty_color: IMG::Color,
}

impl<IMG: Bitmap> Canvas<IMG> {
    fn new(width: u16, height: u16) -> Self {
        let empty_color = IMG::Color::from_rgba(0, 0, 0, 0);
        Self {
            inner: IMG::new(width, height, empty_color),
            empty_color,
        }
    }
    fn clear(&mut self) {
        self.inner = IMG::new(
            self.width(),
            self.height(),
            self.empty_color
        );
    }
    fn resize(&mut self, width: u16, height: u16) {
        // TODO: it's clearing the image, but it shouldn't
        self.inner = IMG::new(width, height, self.empty_color);
    }
    fn set_pixel(&mut self, x: u16, y: u16, color: IMG::Color) {
        self.inner.set_pixel(x, y, color);
    }
    fn bucket(&mut self, x: u16, y: u16, color: IMG::Color) {
        let old_color = self.inner.pixel(x, y);
        self.bucket_recursive(x, y, old_color, color);
    }
    fn bucket_recursive(&mut self, x: u16, y: u16, old_color: IMG::Color, new_color: IMG::Color) {
        self.set_pixel(x, y, new_color);

        for (nx, ny) in self.neighbors(x, y) {
            if self.inner.pixel(nx, ny) == old_color {
                self.bucket_recursive(nx, ny, old_color, new_color);
            }
        }
    }
    fn neighbors(&self, x: u16, y: u16) -> Vec<(u16, u16)> {
        let mut neighbors = Vec::new();
        let w = self.inner.width();
        let h = self.inner.height();

        if x + 1 < w {
            neighbors.push((x + 1, y));
        }
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y + 1 < h {
            neighbors.push((x, y + 1));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
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
