use macroquad::prelude::*;

mod bg;
mod graphics;
mod gui;
mod input;
mod mouse;
mod project;
mod resource;
mod ui_state;
mod util;
mod wrapped_image;

const VERSION: &str = env!("CARGO_PKG_VERSION");

use resource::Resources;
use ui_state::{Effect, UiEvent, UiState, WINDOW_H, WINDOW_W};
use util::*;

fn window_conf() -> Conf {
    Conf {
        window_title: format!("Tarsila {VERSION}: Pixel Art and 2D Sprite Editor"),
        window_width: WINDOW_W,
        window_height: WINDOW_H,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = UiState::default();
    let mut frame = 0;

    loop {
        state.update(frame);
        state.draw();
        next_frame().await;

        if state.must_exit() {
            break;
        }

        frame += 1;
    }
}
