use crate::{util, Effect, Resources};
use egui::Color32;
use lapix::{Event, Size, Tool};
use macroquad::prelude::*;
use std::collections::HashMap;

const TOOL_BTN_IMG_SIZE: Size<usize> = Size { x: 16, y: 16 };
const TOOLS: [Tool; 9] = [
    Tool::Brush,
    Tool::Bucket,
    Tool::Eraser,
    Tool::Eyedropper,
    Tool::Line,
    Tool::Selection,
    Tool::Move,
    Tool::Rectangle,
    Tool::Ellipse,
];

pub struct Toolbar {
    tools: HashMap<Tool, ToolButton>,
    brush: [u8; 3],
    brush_alpha: String,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            tools: TOOLS.iter().map(|t| (*t, ToolButton::new(*t))).collect(),
            brush: [0, 0, 0],
            brush_alpha: "255".to_owned(),
        }
    }

    pub fn sync(&mut self, main_color: [u8; 4]) {
        self.brush = util::rgba_to_rgb_u8(main_color);
        self.brush_alpha = main_color[3].to_string();
    }

    pub fn get_mut(&mut self, tool: Tool) -> Option<&mut ToolButton> {
        self.tools.get_mut(&tool)
    }

    pub fn update(&mut self, egui_ctx: &egui::Context, selected_tool: Tool) -> Vec<Effect> {
        let mut events = Vec::new();

        egui::Window::new("Toolbox")
            //            .default_pos((15., 280.))
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    let colorpicker = ui.color_edit_button_srgb(&mut self.brush);
                    let label = ui.label("a:");
                    let text_edit = ui
                        .add(
                            egui::widgets::TextEdit::singleline(&mut self.brush_alpha)
                                .desired_width(30.0),
                        )
                        .labelled_by(label.id);
                    let color = [
                        self.brush[0],
                        self.brush[1],
                        self.brush[2],
                        self.brush_alpha.parse().unwrap_or(255),
                    ];

                    if colorpicker.changed() || text_edit.changed() {
                        events.push(Event::SetMainColor(color.into()).into());
                    }

                    let btn = ui.button("+").on_hover_text("add to palette");
                    if btn.clicked() {
                        events.push(Event::AddToPalette(color.into()).into());
                    }
                });

                ui.horizontal_wrapped(|ui| {
                    ui.set_max_width(160.);
                    for tool in TOOLS {
                        if let Some(btn) = self.get_mut(tool) {
                            btn.add_to_ui(ui, selected_tool == tool, || {
                                events.push(Event::SetTool(tool).into())
                            });
                        }
                    }
                });
            });

        events
    }
}

pub struct ToolButton {
    tool: Tool,
    image: egui::ColorImage,
    texture: Option<egui::TextureHandle>,
}

impl ToolButton {
    pub fn new(tool: Tool) -> Self {
        let bytes = Resources::tool_icon(tool);
        let img = Image::from_file_with_format(bytes, None);

        let x = TOOL_BTN_IMG_SIZE.x;
        let y = TOOL_BTN_IMG_SIZE.y;
        let image = egui::ColorImage::from_rgba_unmultiplied([x, y], &img.bytes);

        Self {
            tool,
            image,
            texture: None,
        }
    }

    pub fn add_to_ui<F: FnMut()>(&mut self, ui: &mut egui::Ui, selected: bool, mut action: F) {
        let tooltip: &'static str = self.tooltip();

        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("", self.image.clone(), Default::default())
        });
        let prev_bg_fill = ui.style().visuals.widgets.inactive.weak_bg_fill;
        // Highlight the currently selected tool.
        //
        // FIXME: Ui::scope destroys the toolbar's wrapping layout, so we're forced to temporarily
        // set the style and then set back the old style manually after we're done.
        if selected {
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(218, 218, 218);
        }
        if ui
            .add(egui::ImageButton::new(texture, texture.size_vec2()))
            .on_hover_text(tooltip)
            .clicked()
        {
            (action)();
        }
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = prev_bg_fill;
    }

    // TODO: the shortcut being hardcoded here is a problem since it's
    // configurable
    fn tooltip(&self) -> &'static str {
        match self.tool {
            Tool::Brush => "brush tool (B)",
            Tool::Bucket => "bucket tool (G)",
            Tool::Eraser => "eraser tool (E)",
            Tool::Eyedropper => "eyedropper tool (I)",
            Tool::Line => "line tool (L)",
            Tool::Selection => "selection tool (S)",
            Tool::Move => "move tool (M)",
            Tool::Rectangle => "rectangle tool (R)",
            Tool::Ellipse => "ellipse tool (O)",
        }
    }
}
