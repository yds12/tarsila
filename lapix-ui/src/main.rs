use lapix_core::{Bitmap, Event, Tool};
use macroquad::prelude::*;

mod gui;
mod keyboard;
mod ui_state;
mod wrapped_image;

use ui_state::{UiState, WINDOW_H, WINDOW_W};

fn window_conf() -> Conf {
    Conf {
        window_title: "Lapix: Pixel Art and 2D Sprite Editor".to_owned(),
        window_width: WINDOW_W,
        window_height: WINDOW_H,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = UiState::default();

    loop {
        clear_background(SKYBLUE);
        state.update();

        let (x, y) = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = state.screen_to_canvas(x, y);

            if x >= 0
                && y >= 0
                && (x as u16) < state.canvas().width()
                && (y as u16) < state.canvas().height()
            {
                match state.selected_tool() {
                    Tool::Brush => {
                        state.execute(Event::BrushStart);
                    }
                    Tool::Eraser => {
                        state.execute(Event::EraseStart);
                    }
                    Tool::Line => {
                        state.execute(Event::LineStart(x as u16, y as u16));
                    }
                    Tool::Eyedropper => {
                        let color = state.canvas().inner().pixel(x as u16, y as u16);
                        state.execute(Event::SetMainColor(color));
                        state.execute(Event::SetTool(Tool::Brush));
                    }
                    Tool::Bucket => {
                        state.execute(Event::Bucket(x as u16, y as u16));
                    }
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = state.screen_to_canvas(x, y);

            if x >= 0
                && y >= 0
                && (x as u16) < state.canvas().width()
                && (y as u16) < state.canvas().height()
            {
                match state.selected_tool() {
                    Tool::Brush => {
                        state.execute(Event::BrushStroke(x as u16, y as u16));
                    }
                    Tool::Eraser => {
                        state.execute(Event::Erase(x as u16, y as u16));
                    }
                    _ => (),
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            let (x, y) = state.screen_to_canvas(x, y);

            if x >= 0
                && y >= 0
                && (x as u16) < state.canvas().width()
                && (y as u16) < state.canvas().height()
            {
                match state.selected_tool() {
                    Tool::Brush => {
                        state.execute(Event::BrushEnd);
                    }
                    Tool::Eraser => {
                        state.execute(Event::EraseEnd);
                    }
                    Tool::Line => {
                        state.execute(Event::LineEnd(x as u16, y as u16));
                    }
                    _ => (),
                }
            }
        }

        state.process_shortcuts();
        state.draw();
        next_frame().await
    }
}
