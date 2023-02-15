use crate::color::{BLACK, TRANSPARENT};
use crate::{
    graphics, util, Bitmap, Canvas, CanvasEffect, Color, Event, FreeImage, Layer, Layers, Palette,
    Point, Position, Rect, Size, Tool,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Selection {
    Canvas(Rect<i32>),
    FreeImage,
}

pub struct State<IMG: Bitmap> {
    layers: Layers<IMG>,
    events: Vec<Event>,
    tool: Tool,
    main_color: Color,
    spritesheet: Size<u8>,
    palette: Palette,
    selection: Option<Selection>,
    free_image: Option<FreeImage<IMG>>,
    clipboard: Option<IMG>,
}

impl<IMG: Bitmap> State<IMG> {
    pub fn new(size: Size<i32>) -> Self {
        Self {
            layers: Layers::new(size),
            events: Vec::new(),
            tool: Tool::Brush,
            main_color: BLACK,
            spritesheet: Size::new(1, 1),
            palette: Palette::default(),
            selection: None,
            free_image: None,
            clipboard: None,
        }
    }

    pub fn execute(&mut self, event: Event) -> CanvasEffect {
        if let Some(prev_event) = self.events.last() {
            if (prev_event == &event && !event.repeatable())
                || (event.same_variant(prev_event) && !event.type_repeatable())
            {
                return CanvasEffect::None;
            }
        }

        dbg!(&event);
        let t0 = std::time::SystemTime::now();

        if event.triggers_anchoring() {
            self.anchor();
        }

        match event.clone() {
            Event::ClearCanvas => self.canvas_mut().clear(),
            Event::ResizeCanvas(size) => self.resize_canvas(size),
            Event::BrushStart | Event::LineStart(_) | Event::EraseStart | Event::RectStart(_) => {
                self.canvas_mut().start_editing_bundle()
            }
            Event::BrushEnd | Event::EraseEnd => self.canvas_mut().finish_editing_bundle(),
            Event::LineEnd(p) => {
                let last_event = self.events.last();
                let p0 = match last_event {
                    Some(Event::LineStart(p0)) => *p0,
                    _ => panic!("line not started!"),
                };
                let color = self.main_color;
                self.canvas_mut().line(p0, p, color);
                self.canvas_mut().finish_editing_bundle();
                self.free_image = None;
            }
            Event::RectEnd(p) => {
                let last_event = self.events.last();
                let p0: Point<i32> = match last_event {
                    Some(Event::RectStart(p0)) => *p0,
                    _ => panic!("rectangle not started!"),
                };
                let color = self.main_color;
                self.canvas_mut().rectangle(p0, p, color);
                self.canvas_mut().finish_editing_bundle();
                self.free_image = None;
            }
            Event::BrushStroke(p) => {
                let last_event = self.events.last();

                match last_event {
                    Some(Event::BrushStroke(p0)) => {
                        let color = self.main_color;
                        let p0 = *p0;
                        self.canvas_mut().line(p0, p, color);
                    }
                    Some(Event::BrushStart) => {
                        let color = self.main_color;
                        self.canvas_mut().set_pixel(p, color);
                    }
                    _ => (),
                }
            }
            Event::Erase(p) => {
                let last_event = self.events.last();

                match last_event {
                    Some(Event::Erase(p0)) => {
                        let p0 = *p0;
                        self.canvas_mut().line(p0, p, TRANSPARENT);
                    }
                    Some(Event::EraseStart) => {
                        self.canvas_mut().set_pixel(p, TRANSPARENT);
                    }
                    _ => (),
                }
            }
            Event::SetTool(tool) => self.tool = tool,
            Event::SetMainColor(color) => self.main_color = color,
            Event::Save(path) => self.save_image(path.to_string_lossy().as_ref()),
            Event::OpenFile(path) => self.open_image(path.to_string_lossy().as_ref()),
            Event::LoadPalette(path) => {
                self.palette = Palette::from_file(path.to_string_lossy().as_ref())
            }
            Event::AddToPalette(color) => self.palette.add_color(color),
            Event::RemoveFromPalette(color) => self.palette.remove_color(color),
            Event::Bucket(p) => {
                if self.canvas().is_in_bounds(p) {
                    self.canvas_mut().start_editing_bundle();
                    let color = self.main_color;
                    self.canvas_mut().bucket(p, color);
                    self.canvas_mut().finish_editing_bundle();
                }
            }
            Event::ClearSelection => (),
            Event::StartSelection(_) => (),
            Event::EndSelection(p) => {
                let last_event = self.events.last();

                if let Some(Event::StartSelection(p0)) = last_event {
                    let size = p.abs_diff(*p0);
                    let corner = p.rect_min_corner(*p0);
                    let rect = Rect::new(corner.x, corner.y, size.x + 1, size.y + 1);
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
                Some(Selection::Canvas(rect)) => {
                    self.canvas_mut().set_area(rect, TRANSPARENT);
                }
                Some(Selection::FreeImage) => {
                    self.free_image = None;
                    self.set_selection(None);
                }
                _ => (),
            },
            Event::MoveStart(p) => match self.selection {
                Some(Selection::Canvas(_)) => {
                    self.free_image_from_selection(Some(p));
                }
                Some(Selection::FreeImage) => {
                    if let Some(free_image) = self.free_image.as_mut() {
                        free_image.pivot = Some(p - free_image.rect.pos());
                    }
                }
                None => (),
            },
            Event::MoveEnd(p) => {
                let last_event = self.events.last();

                if let Some(Event::MoveStart(_)) = last_event {
                    self.move_free_image(p);
                }
            }
            Event::Paste(p) => {
                if let Some(img) = self.clipboard.as_ref().map(|c| c.clone()) {
                    let img = FreeImage::new(p, img);
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
            Event::NewLayerAbove => self.layers.add_above(),
            Event::NewLayerBelow => todo!(),
            Event::SwitchLayer(i) => self.layers.switch_to(i),
            Event::ChangeLayerVisibility(i, visible) => self.layers.set_visibility(i, visible),
            Event::ChangeLayerOpacity(i, alpha) => self.layers.set_opacity(i, alpha),
            // TODO: this should not only remove it, as we need to be able to
            // undo this
            Event::DeleteLayer(i) => self.layers.delete(i),
            Event::SetSpritesheet(size) => self.set_spritesheet(size),
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

    pub fn resize_canvas(&mut self, size: Size<i32>) {
        self.layers.resize_all(size);
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<IMG> {
        self.layers.active_canvas_mut()
    }

    pub fn canvas(&self) -> &Canvas<IMG> {
        self.layers.active_canvas()
    }

    pub fn layers(&self) -> &Layers<IMG> {
        &self.layers
    }

    pub fn selected_tool(&self) -> Tool {
        self.tool
    }

    pub fn main_color(&self) -> Color {
        self.main_color
    }

    pub fn spritesheet(&self) -> Size<u8> {
        self.spritesheet
    }

    fn set_spritesheet(&mut self, size: Size<u8>) {
        if self.canvas().width() % size.x as i32 != 0 || self.canvas().height() % size.y as i32 != 0
        {
            eprintln!("WARN: Canvas size should be a multiple of the spritesheet size");
            return;
        }

        self.spritesheet = size;
    }

    pub fn palette(&self) -> &[Color] {
        self.palette.inner()
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
            Some(Event::MoveStart(_)) => self.move_free_image(mouse_canvas),
            Some(Event::LineStart(p)) => self.update_line_preview(*p, mouse_canvas),
            Some(Event::RectStart(p)) => self.update_rect_preview(*p, mouse_canvas),
            _ => (),
        }
    }

    fn move_free_image(&mut self, new: Position<i32>) {
        if let Some(free_image) = self.free_image.as_mut() {
            free_image.move_by_pivot(new);
            self.set_selection(Some(Selection::FreeImage));
        }
    }

    fn free_image_from_selection(&mut self, mouse_pos: Option<Point<i32>>) {
        if let Some(Selection::Canvas(rect)) = self.selection {
            self.free_image = Some(FreeImage::from_canvas_area(
                &self.canvas(),
                rect.into(),
                mouse_pos.map(|p| p - rect.pos()),
            ));
            self.canvas_mut().set_area(rect, TRANSPARENT);
            self.selection = Some(Selection::FreeImage);
        }
    }

    fn update_line_preview(&mut self, p0: Point<i32>, p: Point<i32>) {
        self.free_image = FreeImage::line_preview(p0, p, self.main_color());
    }

    fn update_rect_preview(&mut self, p0: Point<i32>, p: Point<i32>) {
        self.free_image = FreeImage::rect_preview(p0, p, self.main_color());
    }

    fn save_image(&self, path: &str) {
        let blended = self.layers.blended();
        util::save_image(blended, path);
    }

    fn open_image(&mut self, path: &str) {
        let img = util::load_img_from_file(path);
        self.resize_canvas((img.width() as i32, img.height() as i32).into());

        for (x, y, pixel) in img.enumerate_pixels() {
            let color = Color::new(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
            self.canvas_mut()
                .set_pixel((x as i32, y as i32).into(), color);
        }
    }

    pub fn sprite_images(&self) -> Vec<IMG> {
        let mut imgs = Vec::new();
        let w = self.layers.canvas_at(0).width() / self.spritesheet.x as i32;
        let h = self.layers.canvas_at(0).height() / self.spritesheet.y as i32;

        for j in 0..self.spritesheet.y {
            for i in 0..self.spritesheet.x {
                imgs.push(
                    // TODO: maybe this (and other) multiplication can overflow
                    self.layers
                        .blended_area((i as i32 * w, j as i32 * h, w, h).into()),
                );
            }
        }

        imgs
    }
}
