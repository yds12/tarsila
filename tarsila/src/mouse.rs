use crate::{Effect, UiState};
use lapix::{Bitmap, Event, Tool};
use macroquad::prelude::*;

pub fn update(state: &UiState) -> Vec<Effect> {
    let (x, y) = mouse_position();
    let (x, y) = state.screen_to_canvas(x, y);
    let mut events = Vec::new();

    if is_mouse_button_pressed(MouseButton::Left) {
        if x >= 0
            && y >= 0
            && (x as u16) < state.canvas().width()
            && (y as u16) < state.canvas().height()
        {
            match state.selected_tool() {
                Tool::Brush => {
                    events.push(Event::BrushStart.into());
                }
                Tool::Eraser => {
                    events.push(Event::EraseStart.into());
                }
                Tool::Line => {
                    events.push(Event::LineStart(x as u16, y as u16).into());
                }
                Tool::Eyedropper => {
                    let color = state.visible_pixel(x as u16, y as u16);
                    events.push(Event::SetMainColor(color).into());
                    events.push(Event::SetTool(Tool::Brush).into());
                }
                Tool::Bucket => {
                    events.push(Event::Bucket(x as u16, y as u16).into());
                }
            }
        }
    }

    if is_mouse_button_down(MouseButton::Left) {
        if x >= 0
            && y >= 0
            && (x as u16) < state.canvas().width()
            && (y as u16) < state.canvas().height()
        {
            match state.selected_tool() {
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
        if x >= 0
            && y >= 0
            && (x as u16) < state.canvas().width()
            && (y as u16) < state.canvas().height()
        {
            match state.selected_tool() {
                Tool::Brush => {
                    events.push(Event::BrushEnd.into());
                }
                Tool::Eraser => {
                    events.push(Event::EraseEnd.into());
                }
                Tool::Line => {
                    events.push(Event::LineEnd(x as u16, y as u16).into());
                }
                _ => (),
            }
        }
    }

    events
}
