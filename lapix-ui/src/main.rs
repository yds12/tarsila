use lapix_core::{Bitmap, Event, Tool};
use macroquad::prelude::*;

mod ui_state;
mod wrapped_image;

use lapix_core::primitives::*;
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

fn rgb_to_rgba_u8(color: [u8; 3]) -> [u8; 4] {
    [color[0], color[1], color[2], 255]
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = UiState::default();

    let bytes = include_bytes!("../res/icon/pencil.png");
    let brush_icon = Texture2D::from_file_with_format(&bytes.clone(), None);
    let img = Image::from_file_with_format(bytes, None);
    let egui_brush = egui::ColorImage::from_rgba_unmultiplied([16, 16], &img.bytes);
    let mut brush_texture = None;

    let bytes = include_bytes!("../res/icon/eyedropper.png");
    let eyedropper_icon = Texture2D::from_file_with_format(&bytes.clone(), None);
    let img = Image::from_file_with_format(bytes, None);
    let egui_eyedropper = egui::ColorImage::from_rgba_unmultiplied([16, 16], &img.bytes);
    let mut eyedropper_texture = None;

    let bytes = include_bytes!("../res/icon/bucket.png");
    let bucket_icon = Texture2D::from_file_with_format(&bytes.clone(), None);
    let img = Image::from_file_with_format(bytes, None);
    let egui_bucket = egui::ColorImage::from_rgba_unmultiplied([16, 16], &img.bytes);
    let mut bucket_texture = None;

    let bytes = include_bytes!("../res/icon/eraser.png");
    let eraser_icon = Texture2D::from_file_with_format(&bytes.clone(), None);
    let img = Image::from_file_with_format(bytes, None);
    let egui_eraser = egui::ColorImage::from_rgba_unmultiplied([16, 16], &img.bytes);
    let mut eraser_texture = None;

    let bytes = include_bytes!("../res/icon/line.png");
    let line_icon = Texture2D::from_file_with_format(&bytes.clone(), None);
    let img = Image::from_file_with_format(bytes, None);
    let egui_line = egui::ColorImage::from_rgba_unmultiplied([16, 16], &img.bytes);
    let mut line_texture = None;

    loop {
        clear_background(SKYBLUE);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Canvas").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    let label = ui.label("w:");
                    ui.add(
                        egui::widgets::TextEdit::singleline(state.canvas_w_str())
                            .desired_width(30.0),
                    )
                    .labelled_by(label.id);
                    let label = ui.label("h:");
                    ui.add(
                        egui::widgets::TextEdit::singleline(state.canvas_h_str())
                            .desired_width(30.0),
                    )
                    .labelled_by(label.id);
                });

                let btn = ui.button("New canvas");
                if btn.clicked() {
                    let w: u16 = state.canvas_w_str().parse().unwrap();
                    let h: u16 = state.canvas_h_str().parse().unwrap();
                    state.execute(Event::ResizeCanvas(w, h));
                }

                let brush_mut = state.brush();
                let colorpicker = ui.color_edit_button_srgb(brush_mut);
                if colorpicker.changed() {
                    let color = rgb_to_rgba_u8(*brush_mut);
                    state.execute(Event::SetMainColor(color));
                }

                let btn = ui.button("Erase canvas");
                if btn.clicked() {
                    state.execute(Event::ClearCanvas);
                }
                let btn = ui.button("Save");
                if btn.clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        state.execute(Event::Save(path));
                    }
                }
            });

            egui::Window::new("Toolbox").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    let texture: &egui::TextureHandle = brush_texture.get_or_insert_with(|| {
                        ui.ctx()
                            .load_texture("brush", egui_brush.clone(), Default::default())
                    });
                    if ui
                        .add(egui::ImageButton::new(texture, texture.size_vec2()))
                        .on_hover_text("brush tool (B)")
                        .clicked()
                    {
                        state.execute(Event::SetTool(Tool::Brush));
                    }
                    let texture: &egui::TextureHandle =
                        eyedropper_texture.get_or_insert_with(|| {
                            ui.ctx().load_texture(
                                "eyedropper",
                                egui_eyedropper.clone(),
                                Default::default(),
                            )
                        });
                    if ui
                        .add(egui::ImageButton::new(texture, texture.size_vec2()))
                        .on_hover_text("eyedropper tool (I)")
                        .clicked()
                    {
                        state.execute(Event::SetTool(Tool::Eyedropper));
                    }
                    let texture: &egui::TextureHandle = bucket_texture.get_or_insert_with(|| {
                        ui.ctx()
                            .load_texture("bucket", egui_bucket.clone(), Default::default())
                    });
                    if ui
                        .add(egui::ImageButton::new(texture, texture.size_vec2()))
                        .on_hover_text("bucket tool (G)")
                        .clicked()
                    {
                        state.execute(Event::SetTool(Tool::Bucket));
                    }
                    let texture: &egui::TextureHandle = eraser_texture.get_or_insert_with(|| {
                        ui.ctx()
                            .load_texture("eraser", egui_eraser.clone(), Default::default())
                    });
                    if ui
                        .add(egui::ImageButton::new(texture, texture.size_vec2()))
                        .on_hover_text("eraser tool (E)")
                        .clicked()
                    {
                        state.execute(Event::SetTool(Tool::Eraser));
                    }
                    let texture: &egui::TextureHandle = line_texture.get_or_insert_with(|| {
                        ui.ctx()
                            .load_texture("line", egui_line.clone(), Default::default())
                    });
                    if ui
                        .add(egui::ImageButton::new(texture, texture.size_vec2()))
                        .on_hover_text("line tool (L)")
                        .clicked()
                    {
                        state.execute(Event::SetTool(Tool::Line));
                    }
                });
            });

            if state.selected_tool() == Tool::Eyedropper || state.selected_tool() == Tool::Brush {
                egui_ctx.output().cursor_icon = egui::CursorIcon::None;
            }
        });

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
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
                    _ => (),
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
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
            let (x, y) = mouse_position();
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
                    Tool::Line => {
                        state.execute(Event::LineEnd(x as u16, y as u16));
                    }
                    _ => (),
                }
            }
        }

        // shortcuts
        if is_key_pressed(KeyCode::B) {
            state.execute(Event::SetTool(Tool::Brush));
        }
        if is_key_pressed(KeyCode::I) {
            state.execute(Event::SetTool(Tool::Eyedropper));
        }
        if is_key_pressed(KeyCode::G) {
            state.execute(Event::SetTool(Tool::Bucket));
        }
        if is_key_pressed(KeyCode::E) {
            state.execute(Event::SetTool(Tool::Eraser));
        }
        if is_key_pressed(KeyCode::L) {
            state.execute(Event::SetTool(Tool::Line));
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

        state.draw_canvas_bg();
        state.draw_canvas();
        egui_macroquad::draw();

        if state.selected_tool() == Tool::Eyedropper {
            let (x, y) = mouse_position();
            draw_texture(eyedropper_icon, x, y - eyedropper_icon.height(), 1.);
        } else if state.selected_tool() == Tool::Brush {
            let (x, y) = mouse_position();
            draw_texture(brush_icon, x + 1., y - brush_icon.height() + 2., 1.);
        } else if state.selected_tool() == Tool::Bucket {
            let (x, y) = mouse_position();
            draw_texture(bucket_icon, x + 1., y - bucket_icon.height() + 2., 1.);
        } else if state.selected_tool() == Tool::Eraser {
            let (x, y) = mouse_position();
            draw_texture(eraser_icon, x + 1., y - eraser_icon.height() + 2., 1.);
        }

        next_frame().await
    }
}
