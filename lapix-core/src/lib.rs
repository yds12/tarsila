mod color;

pub use color::Color;

pub struct State<IMG: Bitmap> {
    canvas: Canvas<IMG>,
    tool: Tool,
    main_color: IMG::Color,
}

impl<IMG: Bitmap> State<IMG> {
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
}

#[derive(Debug, Clone)]
pub enum Event<IMG: Bitmap> {
    ClearCanvas,
    ResizeCanvas(u16, u16),
    BrushOnPixel(u16, u16),
    SetTool(Tool),
    SetMainColor(IMG::Color),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tool {
    Brush,
    Eraser,
    Eyedropper,
}

pub trait Bitmap {
    type Color: Color;

    fn new(width: u16, height: u16, color: Self::Color) -> Self;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn pixel(&self, x: u16, y: u16) -> Self::Color;
    fn set_pixel(&mut self, x: u16, y: u16, color: Self::Color);
}

pub struct Canvas<IMG: Bitmap> {
    inner: IMG,
}

impl<IMG: Bitmap> Canvas<IMG> {
    fn new(width: u16, height: u16) -> Self {
        Self {
            inner: IMG::new(width, height, IMG::Color::from_rgb(255, 255, 255)),
        }
    }
    fn clear(&mut self) {
        self.inner = IMG::new(
            self.width(),
            self.height(),
            IMG::Color::from_rgb(255, 255, 255),
        );
    }
    fn resize(&mut self, width: u16, height: u16) {
        // TODO: it's clearing the image, but it shouldn't
        self.inner = IMG::new(width, height, IMG::Color::from_rgb(255, 255, 255));
    }
    fn set_pixel(&mut self, x: u16, y: u16, color: IMG::Color) {
        self.inner.set_pixel(x, y, color);
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
