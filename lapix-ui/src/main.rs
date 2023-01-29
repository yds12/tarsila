use lapix_core::{Bitmap, Event, State, Tool};
use macroquad::prelude::*;

const WINDOW_W: i32 = 1000;
const WINDOW_H: i32 = 600;
const CANVAS_W: u16 = 64;
const CANVAS_H: u16 = 64;
const CANVAS_SCALE: f32 = 8.;
const LEFT_TOOLBAR_W: u16 = 300;

// Center on the space after the toolbar
const CANVAS_X: f32 = LEFT_TOOLBAR_W as f32 + ((WINDOW_W as u16 - LEFT_TOOLBAR_W) / 2) as f32
    - (CANVAS_W as f32 * CANVAS_SCALE / 2.);
const CANVAS_Y: f32 = (WINDOW_H / 2) as f32 - (CANVAS_H as f32 * CANVAS_SCALE / 2.);

struct WrappedImage(pub Image);

impl Bitmap for WrappedImage {
    type Color = [u8; 4];

    fn new(width: u16, height: u16, color: Self::Color) -> Self {
        let bytes = vec![0; width as usize * height as usize * 4];
        let mut img = Self(Image {
            bytes,
            width,
            height,
        });

        for i in 0..width {
            for j in 0..height {
                img.set_pixel(i, j, color);
            }
        }

        img
    }

    fn width(&self) -> u16 {
        self.0.width() as u16
    }
    fn height(&self) -> u16 {
        self.0.height() as u16
    }
    fn pixel(&self, x: u16, y: u16) -> Self::Color {
        let base_idx = y * 4 * self.width() + x * 4;
        [
            self.0.bytes[base_idx as usize],
            self.0.bytes[(base_idx + 1) as usize],
            self.0.bytes[(base_idx + 2) as usize],
            self.0.bytes[(base_idx + 3) as usize],
        ]
    }
    fn set_pixel(&mut self, x: u16, y: u16, color: Self::Color) {
        let base_idx = y * 4 * self.width() + x * 4;
        self.0.bytes[base_idx as usize] = color[0];
        self.0.bytes[(base_idx + 1) as usize] = color[1];
        self.0.bytes[(base_idx + 2) as usize] = color[2];
        self.0.bytes[(base_idx + 3) as usize] = color[3];
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Lapix: Pixel Art and 2D Sprite Editor".to_owned(),
        window_width: WINDOW_W,
        window_height: WINDOW_H,
        high_dpi: true,
        ..Default::default()
    }
}

fn draw_canvas(texture: Texture2D) {
    let w = texture.width();
    let h = texture.height();

    let scale = CANVAS_SCALE;
    let pos_x = CANVAS_X;
    let pos_y = CANVAS_Y;

    let params = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: w * scale,
            y: h * scale,
        }),
        ..Default::default()
    };

    draw_texture_ex(texture, pos_x, pos_y, WHITE, params);
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

fn screen_to_canvas(x: f32, y: f32) -> (i16, i16) {
    (
        ((x - CANVAS_X) / CANVAS_SCALE) as i16,
        ((y - CANVAS_Y) / CANVAS_SCALE) as i16,
    )
}

fn rgb_f32_to_rgba_u8(color: [f32; 3]) -> [u8; 4] {
    [
        (color[0] as f64 * 255_f64).round() as u8,
        (color[1] as f64 * 255_f64).round() as u8,
        (color[2] as f64 * 255_f64).round() as u8,
        255,
    ]
}

fn rgb_to_rgba_u8(color: [u8; 3]) -> [u8; 4] {
    [color[0], color[1], color[2], 255]
}

fn rgba_to_rgb_u8(color: [u8; 4]) -> [u8; 3] {
    [color[0], color[1], color[2]]
}

fn rgb_to_tuple_u8(color: (u8, u8, u8)) -> [u8; 3] {
    [color.0, color.1, color.2]
}

#[macroquad::main(window_conf)]
async fn main() {
    let eyedropper_icon = include_bytes!("../res/icon/eyedropper.png");
    let eyedropper_icon = Texture2D::from_file_with_format(eyedropper_icon, None);

    let mut brush = [0, 0, 0];
    let mut state = State::<WrappedImage>::new(CANVAS_W, CANVAS_H);
    let mut canvas_w_str = state.canvas().width().to_string();
    let mut canvas_h_str = state.canvas().height().to_string();
    let mut drawing = Texture2D::from_image(&state.canvas().inner().0);
    drawing.set_filter(FilterMode::Nearest);

    loop {
        clear_background(SKYBLUE);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Canvas").show(egui_ctx, |ui| {
                let label = ui.label("w:");
                ui.text_edit_singleline(&mut canvas_w_str)
                    .labelled_by(label.id);
                let label = ui.label("h:");
                ui.text_edit_singleline(&mut canvas_h_str)
                    .labelled_by(label.id);

                let btn = ui.button("New canvas");
                if btn.clicked() {
                    let w: u16 = canvas_w_str.parse().unwrap();
                    let h: u16 = canvas_w_str.parse().unwrap();
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
            });

            egui::Window::new("Toolbox").show(egui_ctx, |ui| {
                let btn = ui.button("Brush");
                if btn.clicked() {
                    state.execute(Event::SetTool(Tool::Brush));
                }
                let btn = ui.button("Eyedropper");
                if btn.clicked() {
                    state.execute(Event::SetTool(Tool::Eyedropper));
                }
            });

            if state.selected_tool() == Tool::Eyedropper {
                egui_ctx.output().cursor_icon = egui::CursorIcon::None;
            }
        });

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let (x, y) = screen_to_canvas(x, y);

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

        draw_canvas(drawing);
        egui_macroquad::draw();

        if state.selected_tool() == Tool::Eyedropper {
            let (x, y) = mouse_position();
            draw_texture(eyedropper_icon, x, y - eyedropper_icon.height(), 1.);
        }

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_conversion() {
        assert_eq!(rgb_f32_to_rgba_u8([1., 1., 1.]), [255, 255, 255, 255]);
        assert_eq!(rgb_f32_to_rgba_u8([0., 0., 0.]), [0, 0, 0, 255]);
        assert_eq!(rgb_f32_to_rgba_u8([0.5, 0.5, 0.5]), [128, 128, 128, 255]);
    }
}
