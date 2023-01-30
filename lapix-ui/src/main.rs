use lapix_core::{Bitmap, Event, Tool};
use macroquad::prelude::*;

mod wrapped_image;
mod ui_state;

use ui_state::{UiState, WINDOW_W, WINDOW_H, Direction};

fn window_conf() -> Conf {
    Conf {
        window_title: "Lapix: Pixel Art and 2D Sprite Editor".to_owned(),
        window_width: WINDOW_W,
        window_height: WINDOW_H,
        high_dpi: true,
        ..Default::default()
    }
}

fn draw_canvas_bg(w: f32, h: f32, state: &UiState) {
    let scale = state.zoom();

    let x = state.canvas_pos().x - state.camera().x;
    let y = state.canvas_pos().y - state.camera().y;
    let w = w * scale;
    let h = h * scale;

    let side = 4. * scale;

    // TODO: optimize this by storing a rendered texture that contains all
    // BG rectangles
    let bg1 = Color::new(0.875, 0.875, 0.875, 1.);
    let bg2 = Color::new(0.75, 0.75, 0.75, 1.);
    for i in 0..(w / side + 1.) as usize {
        for j in 0..(h / side + 1.) as usize {
            let cur_w = i as f32 * side;
            let cur_h = j as f32 * side;
            let next_w = (i + 1) as f32 * side;
            let next_h = (j + 1) as f32 * side;
            let x = x + i as f32 * side;
            let y = y + j as f32 * side;
            let w = if next_w <= w { side } else { w - cur_w };
            let h = if next_h <= h { side } else { h - cur_h };
            let color = if (i + j) % 2 == 0 { bg1 } else { bg2 };
            draw_rectangle(x, y, w, h, color);
        }
    }
}

fn draw_canvas(texture: Texture2D, state: &UiState) {
    let w = texture.width();
    let h = texture.height();

    let x = state.canvas_pos().x - state.camera().x;
    let y = state.canvas_pos().y - state.camera().y;
    let scale = state.zoom();

    let params = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: w * scale,
            y: h * scale,
        }),
        ..Default::default()
    };

    draw_texture_ex(texture, x, y, WHITE, params);
}

fn draw_texture(texture: Texture2D, x: f32, y: f32, scale: f32) {
    let w = texture.width();
    let h = texture.height();

    let params = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: w * scale,
            y: h * scale,
        }),
        ..Default::default()
    };

    draw_texture_ex(texture, x, y, WHITE, params);
}

fn screen_to_canvas(x: f32, y: f32, state: &UiState) -> (i16, i16) {
    let canvas_x = state.canvas_pos().x - state.camera().x;
    let canvas_y = state.canvas_pos().y - state.camera().y;
    let scale = state.zoom();

    (
        ((x - canvas_x) / scale) as i16,
        ((y - canvas_y) / scale) as i16,
    )
}

fn rgb_to_rgba_u8(color: [u8; 3]) -> [u8; 4] {
    [color[0], color[1], color[2], 255]
}

fn rgba_to_rgb_u8(color: [u8; 4]) -> [u8; 3] {
    [color[0], color[1], color[2]]
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = UiState::default();

    let brush_bytes = include_bytes!("../res/icon/pencil.png");
    let brush_icon = Texture2D::from_file_with_format(&brush_bytes.clone(), None);
    let img = Image::from_file_with_format(brush_bytes, None);
    let egui_brush = egui::ColorImage::from_rgba_unmultiplied([16, 16], &img.bytes);

    let eyedropper_bytes = include_bytes!("../res/icon/eyedropper.png");
    let eyedropper_icon = Texture2D::from_file_with_format(&eyedropper_bytes.clone(), None);
    let img = Image::from_file_with_format(eyedropper_bytes, None);
    let egui_eyedropper = egui::ColorImage::from_rgba_unmultiplied([16, 16], &img.bytes);

    let mut brush = [0, 0, 0];
    let mut canvas_w_str = state.canvas_size().x.to_string();
    let mut canvas_h_str = state.canvas_size().y.to_string();
    let mut drawing = Texture2D::from_image(&state.canvas().inner().0);
    drawing.set_filter(FilterMode::Nearest);

    let mut eyedropper_texture = None;
    let mut brush_texture = None;

    loop {
        clear_background(SKYBLUE);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Canvas").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    let label = ui.label("w:");
                    ui.add(
                        egui::widgets::TextEdit::singleline(&mut canvas_w_str).desired_width(30.0),
                    )
                    .labelled_by(label.id);
                    let label = ui.label("h:");
                    ui.add(
                        egui::widgets::TextEdit::singleline(&mut canvas_h_str).desired_width(30.0),
                    )
                    .labelled_by(label.id);
                });

                let btn = ui.button("New canvas");
                if btn.clicked() {
                    let w: u16 = canvas_w_str.parse().unwrap();
                    let h: u16 = canvas_h_str.parse().unwrap();
                    state.execute(Event::ResizeCanvas(w, h));
                    drawing = Texture2D::from_image(&state.canvas().inner().0);
                    drawing.set_filter(FilterMode::Nearest);
                }

                brush = rgba_to_rgb_u8(state.main_color());
                let colorpicker = ui.color_edit_button_srgb(&mut brush);
                if colorpicker.changed() {
                    state.execute(Event::SetMainColor(rgb_to_rgba_u8(brush)));
                }

                let btn = ui.button("Erase canvas");
                if btn.clicked() {
                    state.execute(Event::ClearCanvas);
                    drawing = Texture2D::from_image(&state.canvas().inner().0);
                    drawing.set_filter(FilterMode::Nearest);
                }
                let btn = ui.button("Save");
                if btn.clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        state.execute(Event::Save(path));
                    }
                }
            });

            egui::Window::new("Toolbox").show(egui_ctx, |ui| {
                ui.horizontal(|ui|{
                    let texture: &egui::TextureHandle = brush_texture.get_or_insert_with(|| {
                        ui.ctx()
                            .load_texture("brush", egui_brush.clone(), Default::default())
                    });
                    if ui
                        .add(egui::ImageButton::new(texture, texture.size_vec2()))
                        .on_hover_text("brush tool")
                        .clicked()
                    {
                        state.execute(Event::SetTool(Tool::Brush));
                    }
                    let texture: &egui::TextureHandle = eyedropper_texture.get_or_insert_with(|| {
                        ui.ctx()
                            .load_texture("eyedropper", egui_eyedropper.clone(), Default::default())
                    });
                    if ui
                        .add(egui::ImageButton::new(texture, texture.size_vec2()))
                        .on_hover_text("eyedropper tool")
                        .clicked()
                    {
                        state.execute(Event::SetTool(Tool::Eyedropper));
                    }
                });
            });

            if state.selected_tool() == Tool::Eyedropper || state.selected_tool() == Tool::Brush {
                egui_ctx.output().cursor_icon = egui::CursorIcon::None;
            }
        });

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let (x, y) = screen_to_canvas(x, y, &state);

            if x >= 0
                && y >= 0
                && (x as u16) < state.canvas().width()
                && (y as u16) < state.canvas().height()
            {
                match state.selected_tool() {
                    Tool::Brush => {
                        state.execute(Event::BrushOnPixel(x as u16, y as u16));
                        drawing.update(&state.canvas().inner().0);
                    }
                    Tool::Eyedropper => {
                        let color = state.canvas().inner().pixel(x as u16, y as u16);
                        state.execute(Event::SetMainColor(color));
                        state.execute(Event::SetTool(Tool::Brush));
                    }
                    _ => todo!(),
                }
            }
        }

        if is_key_pressed(KeyCode::B) {
            state.execute(Event::SetTool(Tool::Brush));
        }
        if is_key_pressed(KeyCode::I) {
            state.execute(Event::SetTool(Tool::Eyedropper));
        }
        if is_key_pressed(KeyCode::Equal) {
            state.zoom_in();
        }
        if is_key_pressed(KeyCode::Minus) {
            state.zoom_out();
        }

        if is_key_down(KeyCode::Left) {
            state.move_camera(Direction::Left);
        }
        if is_key_down(KeyCode::Right) {
            state.move_camera(Direction::Right);
        }
        if is_key_down(KeyCode::Up) {
            state.move_camera(Direction::Up);
        }
        if is_key_down(KeyCode::Down) {
            state.move_camera(Direction::Down);
        }

        draw_canvas_bg(drawing.width(), drawing.height(), &state);
        draw_canvas(drawing, &state);
        egui_macroquad::draw();

        if state.selected_tool() == Tool::Eyedropper {
            let (x, y) = mouse_position();
            draw_texture(eyedropper_icon, x, y - eyedropper_icon.height(), 1.);
        } else if state.selected_tool() == Tool::Brush {
            let (x, y) = mouse_position();
            draw_texture(brush_icon, x + 1., y - brush_icon.height() + 2., 1.);
        }

        next_frame().await
    }
}

