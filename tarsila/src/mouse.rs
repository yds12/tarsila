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
        let (x, y) = (self.mouse_canvas.x, self.mouse_canvas.y);
        let mut events = Vec::new();

        if is_mouse_button_pressed(MouseButton::Left) {
            if self.is_on_canvas {
                match self.selected_tool {
                    Tool::Brush => {
                        events.push(Event::BrushStart.into());
                    }
                    Tool::Eraser => {
                        events.push(Event::EraseStart.into());
                    }
                    Tool::Line => {
                        events.push(Event::LineStart(x as u16, y as u16).into());
                    }
                    Tool::Rectangle => {
                        events.push(Event::RectStart(x as u16, y as u16).into());
                    }
                    Tool::Eyedropper => {
                        let color = self.visible_pixel_on_mouse.unwrap();
                        events.push(Event::SetMainColor(color).into());
                        events.push(Event::SetTool(Tool::Brush).into());
                    }
                    Tool::Bucket => {
                        events.push(Event::Bucket(x as u16, y as u16).into());
                    }
                    Tool::Selection => {
                        events.push(Event::StartSelection(x as u16, y as u16).into());
                    }
                    Tool::Move => {
                        if self.is_on_selection {
                            events.push(Event::MoveStart(x as u16, y as u16).into());
                        } else {
                            events.push(Event::ClearSelection.into());
                            self.on_left_release
                                .push(Event::SetTool(Tool::Selection).into());
                        }
                    }
                }
                // TODO: if there's a selection and click was out of it, cancel
                // selection
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if self.is_on_canvas {
                match self.selected_tool {
                    Tool::Brush => {
                        events.push(Event::BrushStroke(x as u16, y as u16).into());
                    }
                    Tool::Eraser => {
                        events.push(Event::Erase(x as u16, y as u16).into());
                    }
                    _ => (),
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if self.is_on_canvas {
                match self.selected_tool {
                    Tool::Brush => {
                        events.push(Event::BrushEnd.into());
                    }
                    Tool::Eraser => {
                        events.push(Event::EraseEnd.into());
                    }
                    Tool::Line => {
                        events.push(Event::LineEnd(x as u16, y as u16).into());
                    }
                    Tool::Rectangle => {
                        events.push(Event::RectEnd(x as u16, y as u16).into());
                    }
                    Tool::Selection => {
                        events.push(Event::EndSelection(x as u16, y as u16).into());
                        events.push(Event::SetTool(Tool::Move).into());
                    }
                    Tool::Move => {
                        events.push(Event::MoveEnd(x as u16, y as u16).into());
                    }
                    _ => (),
                }
            }

            while let Some(event) = self.on_left_release.pop() {
                events.push(event);
            }
        }

        events
    }
}
