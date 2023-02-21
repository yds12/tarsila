use lapix::{Color, Position, Tool};

pub struct StatusBar {
    mouse_canvas: Position<i32>,
    is_mouse_on_canvas: bool,
    selected_tool: Tool,
    visible_pixel_on_mouse: Option<[u8; 4]>,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            mouse_canvas: Position::ZERO,
            is_mouse_on_canvas: false,
            selected_tool: Tool::Brush,
            visible_pixel_on_mouse: None,
        }
    }

    pub fn sync(
        &mut self,
        mouse_canvas: Position<i32>,
        is_mouse_on_canvas: bool,
        selected_tool: Tool,
        visible_pixel_on_mouse: Option<[u8; 4]>,
    ) {
        self.mouse_canvas = mouse_canvas;
        self.is_mouse_on_canvas = is_mouse_on_canvas;
        self.selected_tool = selected_tool;
        self.visible_pixel_on_mouse = visible_pixel_on_mouse;
    }

    pub fn update(&mut self, egui_ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("my_panel").show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                let text_color = egui::Color32::from_rgb(0, 0, 0);

                ui.colored_label(text_color, self.selected_tool.to_string());

                if self.is_mouse_on_canvas {
                    ui.colored_label(
                        text_color,
                        format!("({},{})", self.mouse_canvas.x + 1, self.mouse_canvas.y + 1),
                    );

                    if let Some(color) = self.visible_pixel_on_mouse {
                        ui.colored_label(
                            text_color,
                            format!("color: {}", Color::from(color).hex()),
                        );
                    }
                }
            });
        });
    }
}
