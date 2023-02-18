use crate::Effect;
use lapix::{Event, Position, Tool};
use macroquad::prelude::*;

pub struct MouseManager {
    mouse_canvas: Position<i32>,
    selected_tool: Tool,
    visible_pixel_on_mouse: Option<[u8; 4]>,
    is_on_selection: bool,
    is_on_canvas: bool,
    on_left_release: Vec<Effect>,
}

impl MouseManager {
    pub fn new() -> Self {
        Self {
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
                    events.push(Event::StartSelection(p).into());
                }
                Tool::Move => {
                    if self.is_on_selection {
                        events.push(Event::MoveStart(p).into());
                    } else {
                        events.push(Event::ClearSelection.into());
                        self.on_left_release
                            .push(Event::SetTool(Tool::Selection).into());
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
                Tool::Selection => {
                    events.push(Event::EndSelection(p).into());
                    events.push(Event::SetTool(Tool::Move).into());
                }
                Tool::Move => {
                    events.push(Event::MoveEnd(p).into());
                }
                _ => (),
            }

            while let Some(event) = self.on_left_release.pop() {
                events.push(event);
            }
        }

        events
    }
}
