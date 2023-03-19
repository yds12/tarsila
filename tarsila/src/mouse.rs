use crate::{graphics, Effect, Resources};
use lapix::{Event, Point, Position, Tool};
use macroquad::prelude::*;
use std::collections::HashMap;

#[allow(clippy::type_complexity)]
pub struct MouseManager {
    cursors: CursorSet,
    mouse_canvas: Position<i32>,
    selected_tool: Tool,
    visible_pixel_on_mouse: Option<[u8; 4]>,
    is_on_selection: bool,
    is_on_canvas: bool,
    on_left_release: Vec<Box<fn(&Self) -> Effect>>,
}

impl MouseManager {
    pub fn new() -> Self {
        Self {
            cursors: CursorSet::new(),
            mouse_canvas: Default::default(),
            selected_tool: Tool::Brush,
            visible_pixel_on_mouse: None,
            is_on_selection: false,
            is_on_canvas: false,
            on_left_release: Default::default(),
        }
    }

    pub fn sync(
        &mut self,
        mouse_canvas: Position<i32>,
        is_on_canvas: bool,
        is_on_selection: bool,
        selected_tool: Tool,
        visible_pixel_on_mouse: Option<[u8; 4]>,
    ) {
        self.is_on_selection = is_on_selection;
        self.is_on_canvas = is_on_canvas;
        self.mouse_canvas = mouse_canvas;
        self.selected_tool = selected_tool;
        self.visible_pixel_on_mouse = visible_pixel_on_mouse;
    }

    pub fn draw(&self, selected_tool: Tool) {
        if let Some(cursor) = self.cursors.0.get(&selected_tool) {
            if self.is_on_canvas {
                cursor.draw();
            }
        }
    }

    pub fn update(&mut self) -> Vec<Effect> {
        let p = (self.mouse_canvas.x, self.mouse_canvas.y).into();
        let mut events = Vec::new();

        if is_mouse_button_pressed(MouseButton::Left) {
            match self.selected_tool {
                Tool::Brush => {
                    events.push(Event::BrushStart.into());
                }
                Tool::Eraser => {
                    events.push(Event::EraseStart.into());
                }
                Tool::Line => {
                    events.push(Event::LineStart(p).into());
                }
                Tool::Rectangle => {
                    events.push(Event::RectStart(p).into());
                }
                Tool::Eyedropper => {
                    if let Some(color) = self.visible_pixel_on_mouse {
                        events.push(Event::SetMainColor(color.into()).into());
                        events.push(Event::SetTool(Tool::Brush).into());
                    }
                }
                Tool::Bucket => {
                    events.push(Event::Bucket(p).into());
                }
                Tool::Selection => {
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
                Tool::Move => {
                    if self.is_on_selection {
                        events.push(Event::MoveStart(p).into());
                    } else {
                        events.push(Event::ClearSelection.into());
                    }
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            match self.selected_tool {
                Tool::Brush => {
                    // TODO: there is a bug on macroquad or egui or miniquad
                    // or miniquad-egui or macroquad-egui where the mouse
                    // release event is not registered when it's done out of
                    // the window (so `is_mouse_button_down` is true even
                    // when the mouse is not pressed).
                    events.push(Event::BrushStroke(p).into());
                }
                Tool::Eraser => {
                    events.push(Event::Erase(p).into());
                }
                _ => (),
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            match self.selected_tool {
                Tool::Brush => {
                    events.push(Event::BrushEnd.into());
                }
                Tool::Eraser => {
                    events.push(Event::EraseEnd.into());
                }
                Tool::Line => {
                    events.push(Event::LineEnd(p).into());
                }
                Tool::Rectangle => {
                    events.push(Event::RectEnd(p).into());
                }
                Tool::Move => {
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

pub struct CursorSet(HashMap<Tool, ToolCursor>);

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

        Self(
            tools
                .iter()
                .map(|(t, offset)| (*t, ToolCursor::new(*t, *offset)))
                .collect(),
        )
    }
}

pub struct ToolCursor {
    texture: Texture2D,
    offset: Point<f32>,
}

impl ToolCursor {
    pub fn new(tool: Tool, offset: Point<f32>) -> Self {
        let bytes = Resources::tool_icon(tool);
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
