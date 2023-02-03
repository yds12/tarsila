use crate::wrapped_image::WrappedImage;
use lapix_core::{Event, Point, Size, Tool};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

mod toolbar;
use toolbar::Toolbar;

pub struct Resources;

impl Resources {
    pub fn tool_icon(tool: Tool) -> &'static [u8] {
        match tool {
            Tool::Brush => include_bytes!("../../res/icon/pencil.png"),
            Tool::Bucket => include_bytes!("../../res/icon/bucket.png"),
            Tool::Eraser => include_bytes!("../../res/icon/eraser.png"),
            Tool::Eyedropper => include_bytes!("../../res/icon/eyedropper.png"),
            Tool::Line => include_bytes!("../../res/icon/line.png"),
        }
    }
}

fn rgb_to_rgba_u8(color: [u8; 3]) -> [u8; 4] {
    [color[0], color[1], color[2], 255]
}

fn rgba_to_rgb_u8(color: [u8; 4]) -> [u8; 3] {
    [color[0], color[1], color[2]]
}

fn draw_texture_helper(texture: Texture2D, x: f32, y: f32, scale: f32) {
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

pub struct Gui {
    toolbar: Toolbar,
    cursors: CursorSet,
    canvas_w_str: String,
    canvas_h_str: String,
    brush: [u8; 3],
    last_file: Option<PathBuf>
}

impl Gui {
    pub fn new(canvas_size: Size<u16>) -> Self {
        Self {
            toolbar: Toolbar::new(),
            cursors: CursorSet::new(),
            canvas_w_str: canvas_size.x.to_string(),
            canvas_h_str: canvas_size.y.to_string(),
            brush: [0, 0, 0],
            last_file: None
        }
    }

    pub fn draw_cursor(&self, selected_tool: Tool) {
        if let Some(cursor) = self.cursors.0.get(&selected_tool) {
            cursor.draw();
        }
    }
}

impl Gui {
    pub fn update(&mut self, main_color: [u8; 4]) -> Vec<Event<WrappedImage>> {
        let mut events = Vec::new();
        self.brush = rgba_to_rgb_u8(main_color);

        egui_macroquad::ui(|egui_ctx| {
            let mut canvas_panel_events = self.update_canvas_panel(egui_ctx);
            events.append(&mut canvas_panel_events);

            let mut toolbar_events = self.toolbar.update(egui_ctx);
            events.append(&mut toolbar_events);

            egui_ctx.output().cursor_icon = egui::CursorIcon::None;
        });

        events
    }

    fn update_canvas_panel(&mut self, egui_ctx: &egui::Context)
    -> Vec<Event<WrappedImage>> {
        let mut events = Vec::new();

        egui::Window::new("Canvas").show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("w:");
                ui.add(
                    egui::widgets::TextEdit::singleline(&mut self.canvas_w_str)
                        .desired_width(30.0),
                )
                .labelled_by(label.id);
                let label = ui.label("h:");
                ui.add(
                    egui::widgets::TextEdit::singleline(&mut self.canvas_h_str)
                        .desired_width(30.0),
                )
                .labelled_by(label.id);
            });

            let btn = ui.button("New canvas");
            if btn.clicked() {
                let w: u16 = self.canvas_w_str.parse().unwrap();
                let h: u16 = self.canvas_h_str.parse().unwrap();
                events.push(Event::ResizeCanvas(w, h));
            }

            let colorpicker = ui.color_edit_button_srgb(&mut self.brush);
            if colorpicker.changed() {
                let color = rgb_to_rgba_u8(self.brush);
                events.push(Event::SetMainColor(color));
            }

            let btn = ui.button("Erase canvas");
            if btn.clicked() {
                events.push(Event::ClearCanvas);
            }
            let btn = ui.button("Save");
            if btn.clicked() {
                let mut dialog = rfd::FileDialog::new();

                if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                    dialog = dialog.set_directory(dir);
                }

                if let Some(path) = dialog.save_file() {
                    self.last_file = Some(path.clone());
                    events.push(Event::Save(path));
                }
            }
            let btn = ui.button("Open");
            if btn.clicked() {
                let mut dialog = rfd::FileDialog::new();

                if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                    dialog = dialog.set_directory(dir);
                }

                if let Some(path) = dialog.pick_file() {
                    self.last_file = Some(path.clone());
                    events.push(Event::OpenFile(path));
                }
            }
        });

        events
    }
}

pub struct CursorSet(HashMap<Tool, ToolCursor>);

impl CursorSet {
    pub fn new() -> Self {
        let tools = [
            (Tool::Brush, (0., 0.).into()),
            (Tool::Bucket, (0., 3.).into()),
            (Tool::Eraser, (0., 0.).into()),
            (Tool::Eyedropper, (0., 0.).into()),
            (Tool::Line, (0., 0.).into()),
        ];

        Self(
            tools
                .iter()
                .map(|(t, offset)| (*t, ToolCursor::new(*t, *offset)))
                .collect(),
        )
    }
}

pub struct ToolCursor {
    texture: Texture2D,
    offset: Point<f32>,
}

impl ToolCursor {
    pub fn new(tool: Tool, offset: Point<f32>) -> Self {
        let bytes = Resources::tool_icon(tool);
        let texture = Texture2D::from_file_with_format(&bytes, None);

        Self { texture, offset }
    }

    pub fn draw(&self) {
        let (x, y) = mouse_position();
        draw_texture_helper(
            self.texture,
            x + self.offset.x,
            y - self.texture.height() + self.offset.y,
            1.,
        )
    }
}
