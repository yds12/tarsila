use lapix::{Color, Position, Size, Tool};

pub struct StatusBar {
    mouse_canvas: Position<i32>,
    is_mouse_on_canvas: bool,
    selected_tool: Tool,
    visible_pixel_on_mouse: Option<[u8; 4]>,
    canvas_size: Size<i32>,
    zoom: f32,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            mouse_canvas: Position::ZERO,
            is_mouse_on_canvas: false,
            selected_tool: Tool::Brush,
            visible_pixel_on_mouse: None,
            canvas_size: Size::ZERO,
            zoom: 1.,
        }
    }

    pub fn sync(
        &mut self,
        mouse_canvas: Position<i32>,
        is_mouse_on_canvas: bool,
        selected_tool: Tool,
        visible_pixel_on_mouse: Option<[u8; 4]>,
        canvas_size: Size<i32>,
        zoom: f32,
    ) {
        self.mouse_canvas = mouse_canvas;
        self.is_mouse_on_canvas = is_mouse_on_canvas;
        self.selected_tool = selected_tool;
        self.visible_pixel_on_mouse = visible_pixel_on_mouse;
        self.canvas_size = canvas_size;
        self.zoom = zoom;
    }

    pub fn update(&mut self, egui_ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("my_panel").show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                let text_color = egui::Color32::from_rgb(0, 0, 0);

                ui.colored_label(
                    text_color,
                    format!("{}x{}", self.canvas_size.x, self.canvas_size.y),
                )
                .on_hover_text("canvas size");
                ui.separator();
                ui.colored_label(text_color, format!("{:.0}%", self.zoom * 100.))
                    .on_hover_text("zoom level");
                ui.separator();
                ui.colored_label(text_color, self.selected_tool.to_string())
                    .on_hover_text("current tool");

                if self.is_mouse_on_canvas {
                    ui.separator();
                    ui.colored_label(
                        text_color,
                        format!("{},{}", self.mouse_canvas.x + 1, self.mouse_canvas.y + 1),
                    )
                    .on_hover_text("cursor position in canvas");

                    if let Some(color) = self.visible_pixel_on_mouse {
                        ui.separator();
                        ui.colored_label(text_color, format!("{}", Color::from(color).hex()))
                            .on_hover_text("color under cursor");
                    }
                }
            });
        });
    }
}
