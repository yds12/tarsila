use crate::{graphics, Effect, Resources};
use lapix::{Event, Point, Position, Tool};
use macroquad::prelude::*;
use std::collections::HashMap;

#[allow(clippy::type_complexity)]
pub struct MouseManager {
    cursors: CursorSet,
    cursor: CursorType,
    mouse_canvas: Position<i32>,
    selected_tool: Tool,
    visible_pixel_on_mouse: Option<[u8; 4]>,
    is_on_selection: bool,
    is_on_canvas: bool,
    on_left_release: Vec<Box<fn(&Self) -> Effect>>,
    is_canvas_blocked: bool,
}

impl MouseManager {
    pub fn new() -> Self {
        Self {
            cursors: CursorSet::new(),
            cursor: CursorType::Tool(Tool::Brush),
            mouse_canvas: Default::default(),
            selected_tool: Tool::Brush,
            visible_pixel_on_mouse: None,
            is_on_selection: false,
            is_on_canvas: false,
            on_left_release: Default::default(),
            is_canvas_blocked: false,
        }
    }

    pub fn sync(
        &mut self,
        mouse_canvas: Position<i32>,
        is_on_canvas: bool,
        is_on_selection: bool,
        selected_tool: Tool,
        visible_pixel_on_mouse: Option<[u8; 4]>,
        is_canvas_blocked: bool,
    ) {
        self.is_on_selection = is_on_selection;
        self.is_on_canvas = is_on_canvas;
        self.mouse_canvas = mouse_canvas;
        self.visible_pixel_on_mouse = visible_pixel_on_mouse;
        self.is_canvas_blocked = is_canvas_blocked;

        if self.selected_tool != selected_tool {
            self.set_cursor(CursorType::Tool(selected_tool));
            self.selected_tool = selected_tool;
        }
    }

    pub fn cursor(&self) -> CursorType {
        self.cursor
    }

    pub fn set_cursor(&mut self, cursor: CursorType) {
        self.cursor = cursor;
    }

    pub fn draw(&self) {
        if let Some(cursor) = self.cursors.0.get(&self.cursor) {
            if self.is_on_canvas {
                cursor.draw();
            }
        }
    }

    pub fn update(&mut self) -> Vec<Effect> {
        let p = (self.mouse_canvas.x, self.mouse_canvas.y).into();
        let mut events = Vec::new();

        if is_mouse_button_pressed(MouseButton::Left) {
            match (self.selected_tool, self.is_canvas_blocked) {
                (Tool::Brush, false) => {
                    events.push(Event::BrushStart.into());
                }
                (Tool::Eraser, false) => {
                    events.push(Event::EraseStart.into());
                }
                (Tool::Line, false) => {
                    events.push(Event::LineStart(p).into());
                }
                (Tool::Rectangle, false) => {
                    events.push(Event::RectStart(p).into());
                }
                (Tool::Eyedropper, false) => {
                    if let Some(color) = self.visible_pixel_on_mouse {
                        events.push(Event::SetMainColor(color.into()).into());
                        events.push(Event::SetTool(Tool::Brush).into());
                    }
                }
                (Tool::Bucket, false) => {
                    events.push(Event::Bucket(p).into());
                }
                (Tool::Selection, _) => {
                    if self.is_on_canvas {
                        events.push(Event::StartSelection(p).into());
                        self.on_left_release.push(Box::new(|mouse: &Self| {
                            let point = (mouse.mouse_canvas.x, mouse.mouse_canvas.y).into();
                            Event::EndSelection(point).into()
                        }));
                        self.on_left_release
                            .push(Box::new(|_| Event::SetTool(Tool::Move).into()));
                    }
                }
                (Tool::Move, _) => {
                    if self.is_on_selection {
                        events.push(Event::MoveStart(p).into());
                    } else if !self.is_canvas_blocked {
                        events.push(Event::ClearSelection.into());
                    }
                }
                _ => (),
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            match (self.selected_tool, self.is_canvas_blocked) {
                (Tool::Brush, false) => {
                    // TODO: there is a bug on macroquad or egui or miniquad
                    // or miniquad-egui or macroquad-egui where the mouse
                    // release event is not registered when it's done out of
                    // the window (so `is_mouse_button_down` is true even
                    // when the mouse is not pressed).
                    events.push(Event::BrushStroke(p).into());
                }
                (Tool::Eraser, false) => {
                    events.push(Event::Erase(p).into());
                }
                _ => (),
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            match (self.selected_tool, self.is_canvas_blocked) {
                (Tool::Brush, false) => {
                    events.push(Event::BrushEnd.into());
                }
                (Tool::Eraser, false) => {
                    events.push(Event::EraseEnd.into());
                }
                (Tool::Line, false) => {
                    events.push(Event::LineEnd(p).into());
                }
                (Tool::Rectangle, false) => {
                    events.push(Event::RectEnd(p).into());
                }
                (Tool::Move, _) => {
                    events.push(Event::MoveEnd(p).into());
                }
                _ => (),
            }

            while !self.on_left_release.is_empty() {
                let f = self.on_left_release.remove(0);
                let event = (f)(self);
                events.push(event);
            }
        }

        events
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CursorType {
    Tool(Tool),
    Pan,
}

pub struct CursorSet(HashMap<CursorType, Cursor>);

impl CursorSet {
    pub fn new() -> Self {
        let tools = [
            (Tool::Brush, (0., -16.).into()),
            (Tool::Bucket, (0., -13.).into()),
            (Tool::Eraser, (0., -16.).into()),
            (Tool::Eyedropper, (0., -16.).into()),
            (Tool::Line, (0., -16.).into()),
            (Tool::Selection, (0., 0.).into()),
            (Tool::Move, (-8., -8.).into()),
            (Tool::Rectangle, (0., -16.).into()),
        ];

        let mut hm: HashMap<_, _> = tools
            .iter()
            .map(|(t, offset)| {
                (
                    CursorType::Tool(*t),
                    Cursor::new(CursorType::Tool(*t), *offset),
                )
            })
            .collect();

        hm.insert(
            CursorType::Pan,
            Cursor::new(CursorType::Pan, (0., 0.).into()),
        );

        Self(hm)
    }
}

pub struct Cursor {
    texture: Texture2D,
    offset: Point<f32>,
}

impl Cursor {
    pub fn new(typ: CursorType, offset: Point<f32>) -> Self {
        let bytes = Resources::cursor(typ);
        let texture = Texture2D::from_file_with_format(bytes, None);

        Self { texture, offset }
    }

    pub fn draw(&self) {
        let (x, y) = mouse_position();
        graphics::draw_texture_helper(
            self.texture,
            (x + self.offset.x, y + self.offset.y).into(),
            1.,
        )
    }
}
