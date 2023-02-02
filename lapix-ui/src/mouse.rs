use crate::wrapped_image::WrappedImage;
use crate::UiState;
use lapix_core::{Bitmap, Event, Tool};
use macroquad::prelude::*;

pub fn update(state: &UiState) -> Vec<Event<WrappedImage>> {
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
                    events.push(Event::BrushStart);
                }
                Tool::Eraser => {
                    events.push(Event::EraseStart);
                }
                Tool::Line => {
                    events.push(Event::LineStart(x as u16, y as u16));
                }
                Tool::Eyedropper => {
                    let color = state.canvas().inner().pixel(x as u16, y as u16);
                    events.push(Event::SetMainColor(color));
                    events.push(Event::SetTool(Tool::Brush));
                }
                Tool::Bucket => {
                    events.push(Event::Bucket(x as u16, y as u16));
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
                    events.push(Event::BrushStroke(x as u16, y as u16));
                }
                Tool::Eraser => {
                    events.push(Event::Erase(x as u16, y as u16));
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
                    events.push(Event::BrushEnd);
                }
                Tool::Eraser => {
                    events.push(Event::EraseEnd);
                }
                Tool::Line => {
                    events.push(Event::LineEnd(x as u16, y as u16));
                }
                _ => (),
            }
        }
    }

    events
}
