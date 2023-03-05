use crate::color::{BLACK, TRANSPARENT};
use crate::{
    util, Action, AtomicAction, Bitmap, Canvas, CanvasEffect, Color, Event, FreeImage, Layers,
    Palette, Point, Position, Rect, Size, Tool,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Selection {
    Canvas(Rect<i32>),
    FreeImage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State<IMG> {
    layers: Layers<IMG>,
    #[serde(skip)]
    events: Vec<Event>,
    tool: Tool,
    main_color: Color,
    spritesheet: Size<u8>,
    palette: Palette,
    selection: Option<Selection>,
    free_image: Option<FreeImage<IMG>>,
    clipboard: Option<IMG>,
    #[serde(skip, default = "Vec::new")]
    reversals: Vec<Action<IMG>>,
    #[serde(skip, default = "Option::default")]
    cur_reversal: Option<Action<IMG>>,
}

impl<IMG: Bitmap + Serialize + for<'de> Deserialize<'de>> State<IMG> {
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
            reversals: Vec::new(),
            cur_reversal: None,
        }
    }

    pub fn start_action(&mut self) {
        self.cur_reversal = Some(Action::default());
    }

    pub fn add_to_action(&mut self, actions: Vec<AtomicAction<IMG>>) {
        if self.cur_reversal.is_none() {
            self.start_action();
        }
        self.cur_reversal.as_mut().unwrap().append(actions);
    }

    pub fn end_action(&mut self) {
        if let Some(action) = self.cur_reversal.take() {
            self.reversals.push(action);
        }
    }

    pub fn single_action(&mut self, action: Action<IMG>) {
        self.end_action();
        self.reversals.push(action);
    }

    pub fn add_to_pixels_action(&mut self, actions: Vec<(Point<i32>, Color)>) {
        let actions = AtomicAction::set_pixel_vec(self.layers.active_index(), actions);
        self.add_to_action(actions);
    }

    pub fn single_pixels_action(&mut self, actions: Vec<(Point<i32>, Color)>) {
        let actions = AtomicAction::set_pixel_vec(self.layers.active_index(), actions);
        self.single_action(actions.into());
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

        let mut skip_event = false;

        match event.clone() {
            Event::ClearCanvas => {
                let img = self.canvas_mut().clear();
                let reversal = AtomicAction::SetLayerCanvas(self.layers.active_index(), img);
                self.start_action();
                self.add_to_action(vec![reversal]);
                self.end_action();
            }
            Event::ResizeCanvas(size) => {
                self.start_action();
                let imgs = self.resize_canvas(size);
                self.add_to_action(
                    imgs.into_iter()
                        .enumerate()
                        .map(|(i, img)| AtomicAction::SetLayerCanvas(i, img))
                        .collect(),
                );
                self.end_action();
            }
            Event::LineStart(_) | Event::RectStart(_) => (),
            Event::BrushStart | Event::EraseStart => self.start_action(),
            Event::BrushEnd | Event::EraseEnd => self.end_action(),
            Event::LineEnd(p) => {
                let last_event = self.events.last();
                let p0 = match last_event {
                    Some(Event::LineStart(p0)) => *p0,
                    _ => panic!("line not started!"),
                };
                let color = self.main_color;
                let reversals = self.canvas_mut().line(p0, p, color);
                self.single_pixels_action(reversals);
                self.free_image = None;
            }
            Event::RectEnd(p) => {
                let last_event = self.events.last();
                let p0: Point<i32> = match last_event {
                    Some(Event::RectStart(p0)) => *p0,
                    _ => panic!("rectangle not started!"),
                };
                let color = self.main_color;
                let reversals = self.canvas_mut().rectangle(p0, p, color);
                self.single_pixels_action(reversals);
                self.free_image = None;
            }
            Event::BrushStroke(p) => {
                let last_event = self.events.last();

                let reversals = match last_event {
                    Some(Event::BrushStroke(p0)) => {
                        let color = self.main_color;
                        let p0 = *p0;
                        self.canvas_mut().line(p0, p, color)
                    }
                    Some(Event::BrushStart) => {
                        let color = self.main_color;
                        self.canvas_mut().set_pixel(p, color).into_iter().collect()
                    }
                    _ => Vec::new(),
                };
                self.add_to_pixels_action(reversals);
            }
            Event::Erase(p) => {
                let last_event = self.events.last();

                let reversals = match last_event {
                    Some(Event::Erase(p0)) => {
                        let p0 = *p0;
                        self.canvas_mut().line(p0, p, TRANSPARENT)
                    }
                    Some(Event::EraseStart) => self
                        .canvas_mut()
                        .set_pixel(p, TRANSPARENT)
                        .into_iter()
                        .collect(),
                    _ => Vec::new(),
                };
                self.add_to_pixels_action(reversals);
            }
            Event::SetTool(tool) => self.tool = tool,
            Event::SetMainColor(color) => self.main_color = color,
            Event::Save(path) => self.save_image(path.to_string_lossy().as_ref()),
            Event::OpenFile(path) => self.import_image(path.to_string_lossy().as_ref()),
            Event::SaveProject(path, f) => {
                let bytes = bincode::serialize(&self).unwrap();
                (f.0)(path, bytes);
            }
            Event::LoadProject(path, f) => {
                let bytes = (f.0)(path);
                *self = bincode::deserialize(&bytes).unwrap();
            }
            Event::LoadPalette(path) => {
                self.palette = Palette::from_file(path.to_string_lossy().as_ref())
            }
            Event::AddToPalette(color) => self.palette.add_color(color),
            Event::RemoveFromPalette(color) => self.palette.remove_color(color),
            Event::Bucket(p) => {
                if self.canvas().is_in_bounds(p) {
                    let color = self.main_color;
                    let reversals = self.canvas_mut().bucket(p, color);
                    self.single_pixels_action(reversals);
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
                    let r = rect.clip_to(self.canvas().rect());
                    self.set_selection(Some(Selection::Canvas(r)));
                }
            }
            Event::Copy => match self.selection {
                Some(Selection::Canvas(rect)) => {
                    self.clipboard = Some(self.canvas().img_from_area(rect))
                }
                Some(Selection::FreeImage) => {
                    self.clipboard = Some(self.free_image.as_ref().unwrap().texture.clone())
                }
                None => (),
            },
            Event::DeleteSelection => match self.selection {
                Some(Selection::Canvas(rect)) => {
                    let reversals = self.canvas_mut().set_area(rect, TRANSPARENT);
                    self.single_pixels_action(reversals);
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
                } else {
                    skip_event = true;
                }
            }
            Event::Paste(p) => {
                if let Some(img) = self.clipboard.as_ref().cloned() {
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
                //Transform::Silhouete.apply(&mut self.free_image.as_mut().unwrap().texture);
            }
            Event::NewLayerAbove => {
                self.layers.add_new_above();
                self.end_action();
                self.cur_reversal = Some(Action::default());
                let i = self.layers.count() - 1;
                self.cur_reversal
                    .as_mut()
                    .unwrap()
                    .push(AtomicAction::DestroyLayer(i));
                self.end_action();
            }
            Event::NewLayerBelow => todo!(),
            Event::SwitchLayer(i) => self.layers.switch_to(i),
            Event::ChangeLayerVisibility(i, visible) => self.layers.set_visibility(i, visible),
            Event::ChangeLayerOpacity(i, alpha) => self.layers.set_opacity(i, alpha),
            // TODO: this should not only remove it, as we need to be able to
            // undo this
            Event::DeleteLayer(i) => {
                let img = self.layers.delete(i);
                self.end_action();
                self.cur_reversal = Some(Action::default());
                self.cur_reversal
                    .as_mut()
                    .unwrap()
                    .push(AtomicAction::CreateLayer(i, img));
                self.end_action();
            }
            Event::MoveLayerDown(i) => self.layers.swap(i, i - 1),
            Event::MoveLayerUp(i) => self.layers.swap(i, i + 1),
            Event::SetSpritesheet(size) => self.set_spritesheet(size),
            Event::Undo => {
                // TODO: we should add UNDO to the events list
                dbg!(t0.elapsed().unwrap());
                return self.undo();
            }
        }

        if event.clears_selection() {
            self.clear_selection();
        }

        dbg!(t0.elapsed().unwrap());

        if skip_event {
            println!("Event skipped");
            CanvasEffect::None
        } else {
            let effect = event.canvas_effect();
            self.events.push(event);

            effect
        }
    }

    pub fn resize_canvas(&mut self, size: Size<i32>) -> Vec<IMG> {
        self.layers.resize_all(size)
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
            println!("Anchoring");
            let reversals = self.canvas_mut().paste_obj(&free_image);
            self.single_pixels_action(reversals);
            self.set_selection(Some(Selection::Canvas(
                free_image.rect.clip_to(self.canvas().rect()),
            )));
        }
    }

    fn undo(&mut self) -> CanvasEffect {
        if let Some(action) = self.reversals.pop() {
            return action.apply(&mut self.layers);
        }

        CanvasEffect::None
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
                self.canvas(),
                rect,
                mouse_pos.map(|p| p - rect.pos()),
            ));
            let reversals = self.canvas_mut().set_area(rect, TRANSPARENT);
            self.single_pixels_action(reversals);
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

    fn import_image(&mut self, path: &str) {
        let img = util::load_img_from_file(path);

        if img.width() as i32 > self.canvas().width()
            || img.height() as i32 > self.canvas().height()
        {
            self.resize_canvas((img.width() as i32, img.height() as i32).into());
        }

        let img: IMG = util::img_from_raw(img);
        let img = FreeImage::new(Point::ZERO, img);
        self.free_image = Some(img);
        self.set_selection(Some(Selection::FreeImage));
    }
}
