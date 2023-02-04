use crate::{Bitmap, Canvas, CanvasEffect, Color, Event, Tool};
use std::fmt::Debug;

pub struct Layer<IMG: Bitmap> {
    canvas: Canvas<IMG>,
    visible: bool,
    opacity: u8,
}

impl<IMG: Bitmap> Layer<IMG> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            canvas: Canvas::new(width, height),
            visible: true,
            opacity: 255,
        }
    }

    pub fn canvas(&self) -> &Canvas<IMG> {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<IMG> {
        &mut self.canvas
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn opacity(&self) -> u8 {
        self.opacity
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.canvas.resize(w, h);
    }
}

pub struct State<IMG: Bitmap> {
    layers: Vec<Layer<IMG>>,
    active_layer: usize,
    events: Vec<Event<IMG>>,
    tool: Tool,
    main_color: IMG::Color,
}

impl<IMG: Bitmap + Debug> State<IMG> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            layers: vec![Layer::new(width, height)],
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
            Event::ResizeCanvas(w, h) => self.resize_canvas(w, h),
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
                let color = self.main_color;
                self.canvas_mut().line(point, (x, y).into(), color);
                self.canvas_mut().finish_tool_action();
            }
            Event::BrushStroke(x, y) => {
                let last_event = self.events.last();

                match last_event {
                    Some(Event::BrushStroke(x0, y0)) => {
                        let color = self.main_color;
                        let p0 = (*x0, *y0).into();
                        self.canvas_mut().line(p0, (x, y).into(), color);
                    }
                    Some(Event::BrushStart) => {
                        let color = self.main_color;
                        self.canvas_mut().set_pixel(x, y, color);
                    }
                    _ => ()
                }
            }
            Event::SetTool(tool) => self.tool = tool,
            Event::SetMainColor(color) => self.main_color = color,
            Event::Save(path) => self.save_image(path.to_string_lossy().as_ref()),
            Event::OpenFile(path) => self.open_image(path.to_string_lossy().as_ref()),
            Event::Bucket(x, y) => {
                self.canvas_mut().start_tool_action();
                let color = self.main_color;
                self.canvas_mut().bucket(x, y, color);
                self.canvas_mut().finish_tool_action();
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
            Event::ChangeLayerVisibility(i, visible) => self.change_layer_visibility(i, visible),
            Event::ChangeLayerOpacity(i, alpha) => self.change_layer_opacity(i, alpha),
            // TODO: this should not only remove it, as we need to be able to
            // undo this
            Event::DeleteLayer(i) => self.delete_layer(i),
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

    pub fn resize_canvas(&mut self, width: u16, height: u16) {
        for layer in self.layers.iter_mut() {
            layer.resize(width, height);
        }
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<IMG> {
        self.layers[self.active_layer].canvas_mut()
    }

    pub fn canvas(&self) -> &Canvas<IMG> {
        self.layers[self.active_layer].canvas()
    }

    pub fn layer(&self, index: usize) -> &Layer<IMG> {
        &self.layers[index]
    }

    pub fn layer_canvas(&self, index: usize) -> &Canvas<IMG> {
        self.layers[index].canvas()
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
        let layer = Layer::<IMG>::new(self.canvas().width(), self.canvas().height());
        self.layers.push(layer);
    }

    fn delete_layer(&mut self, index: usize) {
        // TODO: this should not only remove it, as we need to be able to undo this
        self.layers.remove(index);
    }

    fn change_layer_visibility(&mut self, index: usize, visible: bool) {
        self.layers[index].visible = visible;
    }

    fn change_layer_opacity(&mut self, index: usize, opacity: u8) {
        self.layers[index].opacity = opacity;
    }

    fn undo(&mut self) -> CanvasEffect {
        self.canvas_mut().undo_last()
    }

    fn save_image(&self, path: &str) {
        let blended = self.blend_layers();
        let bytes = blended.bytes();

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

        let img = ImageReader::open(path).unwrap().decode().unwrap();
        let img = img.into_rgba8();
        self.canvas_mut()
            .resize(img.width() as u16, img.height() as u16);

        for (x, y, pixel) in img.enumerate_pixels() {
            let color = IMG::Color::from_rgba(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
            self.canvas_mut().set_pixel(x as u16, y as u16, color);
        }
    }

    fn blend_layers(&self) -> IMG {
        let w = self.layer_canvas(0).width();
        let h = self.layer_canvas(0).height();
        let mut result = IMG::new(w, h, IMG::Color::from_rgba(0, 0, 0, 0));

        for x in 0..w {
            for y in 0..h {
                result.set_pixel(x, y, self.visible_pixel(x, y));
            }
        }
        /*
        // Start with the contents of layer 0 (the bottom layer)
        let mut result = if self.layer(0).visible {
            IMG::from_parts(w, h, self.layer_canvas(0).inner().bytes())
        } else {
            IMG::new(w, h, IMG::Color::from_rgba(0, 0, 0, 0))
        };

        for i in 1..self.layers.len() {
            if !self.layer(i).visible {
                continue;
            }

            // TODO: take opacity into account
            for x in 0..w {
                for y in 0..h {
                    let blend = self
                        .layer_canvas(i)
                        .pixel(x, y)
                        .blend_over(result.pixel(x, y));
                    result.set_pixel(x, y, blend);
                }
            }
        }
        */

        result
    }

    pub fn visible_pixel(&self, x: u16, y: u16) -> IMG::Color {
        let mut result = if self.layer(0).visible {
            self.layer_canvas(0).pixel(x, y)
        } else {
            IMG::Color::from_rgba(0, 0, 0, 0)
        };

        for i in 1..self.layers.len() {
            if !self.layer(i).visible {
                continue;
            }

            result = self.layer_canvas(i).pixel(x, y).blend_over(result);
        }

        result
    }
}
