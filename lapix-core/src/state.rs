use crate::{Bitmap, Canvas, CanvasEffect, Color, Event, Tool};
use std::fmt::Debug;

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
                if self.canvas_mut().pixel(x, y) != IMG::Color::from_rgba(0, 0, 0, 0) {
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
        let bytes = self.canvas().bytes();
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
        self.canvas_mut()
            .resize(img.width() as u16, img.height() as u16);

        for (x, y, pixel) in img.enumerate_pixels() {
            let color = IMG::Color::from_rgba(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
            self.canvas_mut().set_pixel(x as u16, y as u16, color);
        }
    }

    fn undo(&mut self) -> CanvasEffect {
        self.canvas_mut().undo_last()
    }
}
