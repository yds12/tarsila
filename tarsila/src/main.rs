use macroquad::prelude::*;

mod gui;
mod keyboard;
mod mouse;
mod ui_state;
mod util;
mod wrapped_image;

use ui_state::{Effect, UiEvent, UiState, WINDOW_H, WINDOW_W};
use util::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tarsila: Pixel Art and 2D Sprite Editor".to_owned(),
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
        state.update();
        state.draw();
        next_frame().await
    }
}
