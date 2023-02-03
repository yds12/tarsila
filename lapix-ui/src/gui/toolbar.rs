use crate::wrapped_image::WrappedImage;
use lapix_core::{Event, Point, Size, Tool};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use super::Resources;

const TOOL_BTN_IMG_SIZE: Size<usize> = Size { x: 16, y: 16 };
const TOOLS: [Tool; 5] = [
    Tool::Brush,
    Tool::Bucket,
    Tool::Eraser,
    Tool::Eyedropper,
    Tool::Line,
];

fn rgb_to_rgba_u8(color: [u8; 3]) -> [u8; 4] {
    [color[0], color[1], color[2], 255]
}

fn rgba_to_rgb_u8(color: [u8; 4]) -> [u8; 3] {
    [color[0], color[1], color[2]]
}


pub struct Toolbar(HashMap<Tool, ToolButton>);

impl Toolbar {
    pub fn new() -> Self {
        Self(TOOLS.iter().map(|t| (*t, ToolButton::new(*t))).collect())
    }

    pub fn get_mut(&mut self, tool: Tool) -> Option<&mut ToolButton> {
        self.0.get_mut(&tool)
    }

    pub fn update(&mut self, egui_ctx: &egui::Context) -> Vec<Event<WrappedImage>> {
        let mut events = Vec::new();

        egui::Window::new("Toolbox").show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                for tool in TOOLS {
                    if let Some(btn) = self.get_mut(tool) {
                        btn.add_to_ui(ui, || events.push(Event::SetTool(tool)));
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

    pub fn add_to_ui<F: FnMut()>(&mut self, ui: &mut egui::Ui, mut action: F) {
        let tooltip: &'static str = self.tooltip();

        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("", self.image.clone(), Default::default())
        });
        if ui
            .add(egui::ImageButton::new(texture, texture.size_vec2()))
            .on_hover_text(tooltip)
            .clicked()
        {
            (action)();
        }
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
        }
    }
}
