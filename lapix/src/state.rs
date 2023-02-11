use crate::color::{BLACK, TRANSPARENT};
use crate::{
    graphics, Bitmap, Canvas, CanvasEffect, Color, Event, FreeImage, Point, Position, Rect, Size,
    Tool,
};

const MAX_PALETTE: usize = 200;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Selection {
    Canvas(Rect<u16>),
    FreeImage,
}

pub struct State<IMG: Bitmap> {
    layers: Vec<Layer<IMG>>,
    active_layer: usize,
    events: Vec<Event>,
    tool: Tool,
    main_color: Color,
    spritesheet: Size<u8>,
    palette: Vec<Color>,
    selection: Option<Selection>,
    free_image: Option<FreeImage<IMG>>,
    clipboard: Option<IMG>,
}

impl<IMG: Bitmap> State<IMG> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            layers: vec![Layer::new(width, height)],
            active_layer: 0,
            events: Vec::new(),
            tool: Tool::Brush,
            main_color: BLACK,
            spritesheet: Size::new(1, 1),
            palette: vec![
                Color::new(0, 0, 0, 255),       // BLACK
                Color::new(255, 255, 255, 255), // WHITE
                Color::new(255, 0, 0, 255),     // RED
                Color::new(255, 127, 0, 255),   // RED + YELLOW = ORANGE
                Color::new(255, 255, 0, 255),   // YELLOW
                Color::new(127, 255, 0, 255),   // GREEN + YELLOW
                Color::new(0, 255, 0, 255),     // GREEN
                Color::new(0, 255, 127, 255),   // GREEN + CYAN
                Color::new(0, 255, 255, 255),   // CYAN
                Color::new(0, 127, 255, 255),   // BLUE + CYAN
                Color::new(0, 0, 255, 255),     // BLUE
                Color::new(127, 0, 255, 255),   // BLUE + MAGENTA
                Color::new(255, 0, 255, 255),   // MAGENTA
                Color::new(255, 0, 127, 255),   // RED + MAGENTA
            ],
            selection: None,
            free_image: None,
            clipboard: None,
        }
    }

    pub fn execute(&mut self, event: Event) -> CanvasEffect {
        if Some(&event) == self.events.last() && !event.repeatable() {
            return CanvasEffect::None;
        }

        dbg!(&event);
        let t0 = std::time::SystemTime::now();

        if event.triggers_anchoring() {
            self.anchor();
        }

        match event.clone() {
            Event::ClearCanvas => self.canvas_mut().clear(),
            Event::ResizeCanvas(w, h) => self.resize_canvas(w, h),
            Event::BrushStart
            | Event::LineStart(_, _)
            | Event::EraseStart
            | Event::RectStart(_, _) => self.canvas_mut().start_editing_bundle(),
            Event::BrushEnd | Event::EraseEnd => self.canvas_mut().finish_editing_bundle(),
            Event::LineEnd(x, y) => {
                let last_event = self.events.last();
                let point = match last_event {
                    Some(Event::LineStart(i, j)) => (*i, *j).into(),
                    _ => panic!("line not started!"),
                };
                let color = self.main_color;
                self.canvas_mut().line(point, (x, y).into(), color);
                self.canvas_mut().finish_editing_bundle();
                self.free_image = None;
            }
            Event::RectEnd(x, y) => {
                let last_event = self.events.last();
                let p: Point<u16> = match last_event {
                    Some(Event::RectStart(i, j)) => (*i, *j).into(),
                    _ => panic!("rectangle not started!"),
                };
                let color = self.main_color;

                // draw 4 lines
                self.canvas_mut()
                    .line((p.x, p.y).into(), (p.x, y).into(), color);
                self.canvas_mut()
                    .line((p.x, p.y).into(), (x, p.y).into(), color);
                self.canvas_mut()
                    .line((p.x, y).into(), (x, y).into(), color);
                self.canvas_mut()
                    .line((x, p.y).into(), (x, y).into(), color);

                self.canvas_mut().finish_editing_bundle();
                self.free_image = None;
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
                    _ => (),
                }
            }
            Event::Erase(x, y) => {
                let last_event = self.events.last();

                match last_event {
                    Some(Event::Erase(x0, y0)) => {
                        let p0 = (*x0, *y0).into();
                        self.canvas_mut().line(p0, (x, y).into(), TRANSPARENT);
                    }
                    Some(Event::EraseStart) => {
                        self.canvas_mut().set_pixel(x, y, TRANSPARENT);
                    }
                    _ => (),
                }
            }
            Event::SetTool(tool) => self.tool = tool,
            Event::SetMainColor(color) => self.main_color = color,
            Event::Save(path) => self.save_image(path.to_string_lossy().as_ref()),
            Event::OpenFile(path) => self.open_image(path.to_string_lossy().as_ref()),
            Event::LoadPalette(path) => self.load_palette(path.to_string_lossy().as_ref()),
            Event::AddToPalette(color) => {
                if !self.palette.contains(&color) {
                    self.palette.push(color)
                }
            }
            Event::RemoveFromPalette(color) => {
                self.palette.retain(|c| *c != color);
            }
            Event::Bucket(x, y) => {
                self.canvas_mut().start_editing_bundle();
                let color = self.main_color;
                self.canvas_mut().bucket(x, y, color);
                self.canvas_mut().finish_editing_bundle();
            }
            Event::ClearSelection => (),
            Event::StartSelection(_, _) => (),
            Event::EndSelection(x, y) => {
                let last_event = self.events.last();

                if let Some(Event::StartSelection(x0, y0)) = last_event {
                    let (x, y, w, h) = match (*x0, *y0, x, y) {
                        (x0, y0, x, y) if x0 <= x && y0 <= y => (x0, y0, x - x0, y - y0),
                        (x0, y0, x, y) if x0 > x && y0 <= y => (x, y0, x0 - x, y - y0),
                        (x0, y0, x, y) if x0 > x && y0 > y => (x, y, x0 - x, y0 - y),
                        (x0, y0, x, y) if x0 <= x && y0 > y => (x0, y, x - x0, y0 - y),
                        _ => unreachable!(),
                    };

                    let rect = Rect::new(x, y, w + 1, h + 1);
                    self.set_selection(Some(Selection::Canvas(rect)));
                }
            }
            Event::Copy => match self.selection {
                Some(Selection::Canvas(rect)) => {
                    self.clipboard = Some(self.canvas().img_from_area(rect.into()))
                }
                Some(Selection::FreeImage) => {
                    self.clipboard = Some(self.free_image.as_ref().unwrap().texture.clone())
                }
                None => (),
            },
            Event::DeleteSelection => match self.selection {
                Some(Selection::Canvas(rect)) => self.canvas_mut().set_area(rect, TRANSPARENT),
                Some(Selection::FreeImage) => {
                    self.free_image = None;
                    self.set_selection(None);
                }
                _ => (),
            },
            Event::MoveStart(x, y) => match self.selection {
                Some(Selection::Canvas(_)) => {
                    self.free_image_from_selection(Some((x, y).into()));
                }
                Some(Selection::FreeImage) => {
                    if let Some(free_image) = self.free_image.as_mut() {
                        free_image.pivot = Some(
                            (
                                (x as i32 - free_image.rect.x) as u16,
                                (y as i32 - free_image.rect.y) as u16,
                            )
                                .into(),
                        );
                    }
                }
                None => (),
            },
            Event::MoveEnd(x, y) => {
                let last_event = self.events.last();

                if let Some(Event::MoveStart(_, _)) = last_event {
                    self.move_free_image((x as i32, y as i32).into());
                }
            }
            Event::Paste(x, y) => {
                if let Some(img) = self.clipboard.as_ref().map(|c| c.clone()) {
                    let img = FreeImage::new(x as i32, y as i32, img);
                    self.free_image = Some(img);
                    self.set_selection(Some(Selection::FreeImage));
                }
            }
            Event::FlipHorizontal => {
                if let Some(Selection::Canvas(_)) = self.selection {
                    self.free_image_from_selection(None);
                }
                if let Some(free_img) = self.free_image.as_mut() {
                    free_img.flip_horizontally();
                }
            }
            Event::FlipVertical => {
                if let Some(Selection::Canvas(_)) = self.selection {
                    self.free_image_from_selection(None);
                }
                if let Some(free_img) = self.free_image.as_mut() {
                    free_img.flip_vertically();
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
            Event::SetSpritesheet(w, h) => self.set_spritesheet(w, h),
            Event::Undo => {
                // TODO: we should add UNDO to the events list
                dbg!(t0.elapsed().unwrap());
                return self.undo();
            }
            _ => todo!(),
        }

        if event.clears_selection() {
            self.clear_selection();
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

    pub fn main_color(&self) -> Color {
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

    pub fn spritesheet(&self) -> Size<u8> {
        self.spritesheet
    }

    fn set_spritesheet(&mut self, w: u8, h: u8) {
        if self.canvas().width() % w as u16 != 0 || self.canvas().height() % h as u16 != 0 {
            eprintln!("WARN: Canvas size should be a multiple of the spritesheet size");
            return;
        }

        self.spritesheet = Size::new(w, h);
    }

    pub fn palette(&self) -> &[Color] {
        &self.palette
    }

    pub fn selection(&self) -> Option<Selection> {
        self.selection
    }

    pub fn free_image(&self) -> Option<&FreeImage<IMG>> {
        self.free_image.as_ref()
    }

    fn clear_selection(&mut self) {
        self.set_selection(None);
    }

    fn set_selection(&mut self, selection: Option<Selection>) {
        match selection {
            None => self.selection = None,
            s @ Some(Selection::Canvas(_)) => self.selection = s,
            s @ Some(Selection::FreeImage) => {
                if self.free_image.is_none() {
                    panic!("no free image to select");
                }
                self.selection = s;
            }
        }
    }

    fn anchor(&mut self) {
        if let Some(free_image) = self.free_image.take() {
            self.canvas_mut().paste_obj(&free_image);
            self.set_selection(Some(Selection::Canvas(
                free_image.rect.clip_to(self.canvas().rect().into()).into(),
            )));
        }
    }

    fn undo(&mut self) -> CanvasEffect {
        self.canvas_mut().undo_last()
    }

    pub fn update_free_image(&mut self, mouse_canvas: Position<i32>) {
        match self.events.last() {
            Some(Event::MoveStart(_, _)) => self.move_free_image(mouse_canvas),
            Some(Event::LineStart(x, y)) => {
                self.update_line_preview((*x as i32, *y as i32).into(), mouse_canvas)
            }
            Some(Event::RectStart(x, y)) => {
                // TODO: rectangle preview
                self.update_rect_preview((*x as i32, *y as i32).into(), mouse_canvas)
            }
            _ => (),
        }
    }

    fn move_free_image(&mut self, new: Position<i32>) {
        if let Some(free_image) = self.free_image.as_mut() {
            free_image.move_by_pivot(new);
            self.set_selection(Some(Selection::FreeImage));
        }
    }

    fn free_image_from_selection(&mut self, mouse_pos: Option<Point<u16>>) {
        if let Some(Selection::Canvas(rect)) = self.selection {
            self.free_image = Some(FreeImage::from_canvas_area(
                &self.canvas(),
                rect.into(),
                mouse_pos.map(|p| (p.x - rect.x, p.y - rect.y).into()),
            ));
            self.canvas_mut().set_area(rect, TRANSPARENT);
            self.selection = Some(Selection::FreeImage);
        }
    }

    fn update_line_preview(&mut self, p0: Point<i32>, p: Point<i32>) {
        let line = graphics::line(p0, p);
        let (xspan, yspan) = ((p.x - p0.x).abs(), (p.y - p0.y).abs());

        if xspan == 0 && yspan == 0 {
            return;
        }

        let mut img = IMG::new(xspan as u16 + 1, yspan as u16 + 1, TRANSPARENT);

        let offset = Point::new(
            std::cmp::min(p.x, p0.x) as i32,
            std::cmp::min(p.y, p0.y) as i32,
        );
        for point in line {
            let x = (point.x - offset.x) as u16;
            let y = (point.y - offset.y) as u16;
            img.set_pixel(x, y, self.main_color());
        }

        self.free_image = Some(FreeImage::new(offset.x, offset.y, img));
    }

    fn update_rect_preview(&mut self, p0: Point<i32>, p: Point<i32>) {
        let l1 = graphics::line((p0.x, p0.y).into(), (p0.x, p.y).into());
        let l2 = graphics::line((p0.x, p0.y).into(), (p.x, p0.y).into());
        let l3 = graphics::line((p.x, p0.y).into(), (p.x, p.y).into());
        let l4 = graphics::line((p0.x, p.y).into(), (p.x, p.y).into());
        let rect = vec![l1, l2, l3, l4].into_iter().flatten();

        let (xspan, yspan) = ((p.x - p0.x).abs(), (p.y - p0.y).abs());

        if xspan == 0 && yspan == 0 {
            return;
        }

        let mut img = IMG::new(xspan as u16 + 1, yspan as u16 + 1, TRANSPARENT);

        let offset = Point::new(
            std::cmp::min(p.x, p0.x) as i32,
            std::cmp::min(p.y, p0.y) as i32,
        );
        for point in rect {
            let x = (point.x - offset.x) as u16;
            let y = (point.y - offset.y) as u16;
            img.set_pixel(x, y, self.main_color());
        }

        self.free_image = Some(FreeImage::new(offset.x, offset.y, img));
    }

    fn save_image(&self, path: &str) {
        let blended = self.blended_layers();
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
        let img = self.load_img_from_file(path);
        self.canvas_mut()
            .resize(img.width() as u16, img.height() as u16);

        for (x, y, pixel) in img.enumerate_pixels() {
            let color = Color::new(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
            self.canvas_mut().set_pixel(x as u16, y as u16, color);
        }
    }

    fn load_palette(&mut self, path: &str) {
        let img = self.load_img_from_file(path);
        self.palette = Vec::new();

        for (_, _, pixel) in img.enumerate_pixels() {
            let color = Color::new(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);

            if !self.palette.contains(&color) {
                self.palette.push(color);
            }

            if self.palette.len() >= MAX_PALETTE {
                break;
            }
        }
    }

    fn load_img_from_file(&self, path: &str) -> image::RgbaImage {
        use image::io::Reader as ImageReader;
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        img.into_rgba8()
    }

    pub fn blended_layers(&self) -> IMG {
        let w = self.layer_canvas(0).width();
        let h = self.layer_canvas(0).height();

        self.blended_layers_rect(0, 0, w, h)
    }

    pub fn blended_layers_rect(&self, x: u16, y: u16, w: u16, h: u16) -> IMG {
        let mut result = IMG::new(w, h, TRANSPARENT);

        for i in 0..w {
            for j in 0..h {
                result.set_pixel(i, j, self.visible_pixel(x + i, y + j));
            }
        }

        result
    }

    pub fn sprite_images(&self) -> Vec<IMG> {
        let mut imgs = Vec::new();
        let w = self.layer_canvas(0).width() / self.spritesheet.x as u16;
        let h = self.layer_canvas(0).height() / self.spritesheet.y as u16;

        for j in 0..self.spritesheet.y {
            for i in 0..self.spritesheet.x {
                imgs.push(
                    // TODO: maybe this (and other) multiplication can overflow
                    self.blended_layers_rect(i as u16 * w, j as u16 * h, w, h),
                );
            }
        }

        imgs
    }

    pub fn visible_pixel(&self, x: u16, y: u16) -> Color {
        let mut result = if self.layer(0).visible {
            self.layer_canvas(0).pixel(x, y)
        } else {
            TRANSPARENT
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
