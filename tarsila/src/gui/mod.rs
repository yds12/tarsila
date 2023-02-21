use crate::{Effect, UiEvent};
use lapix::{Event, Point, Position, Size, Tool};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

mod layers;
mod menu;
mod palette;
mod preview;
mod status;
mod toolbar;

use layers::LayersPanel;
use menu::MenuBar;
use palette::Palette;
use preview::Preview;
use status::StatusBar;
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
            Tool::Selection => include_bytes!("../../res/icon/selection.png"),
            Tool::Move => include_bytes!("../../res/icon/move.png"),
            Tool::Rectangle => include_bytes!("../../res/icon/rectangle.png"),
        }
    }
}

fn draw_texture_helper(texture: Texture2D, p: Position<f32>, scale: f32) {
    let w = texture.width();
    let h = texture.height();

    let params = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: w * scale,
            y: h * scale,
        }),
        ..Default::default()
    };

    draw_texture_ex(texture, p.x, p.y, WHITE, params);
}

pub struct Gui {
    toolbar: Toolbar,
    layers_panel: LayersPanel,
    cursors: CursorSet,
    canvas_size: (String, String),
    last_file: Option<PathBuf>,
    spritesheet: (String, String),
    preview: Preview,
    palette: Palette,
    status_bar: StatusBar,
    menu: MenuBar,
}

impl Gui {
    pub fn new(canvas_size: Size<u16>) -> Self {
        Self {
            toolbar: Toolbar::new(),
            layers_panel: LayersPanel::new(),
            cursors: CursorSet::new(),
            canvas_size: (canvas_size.x.to_string(), canvas_size.y.to_string()),
            last_file: None,
            spritesheet: ("1".to_owned(), "1".to_owned()),
            preview: Preview::new(),
            palette: Palette::new(),
            status_bar: StatusBar::new(),
            menu: MenuBar::new(),
        }
    }

    pub fn draw_cursor(&self, selected_tool: Tool) {
        if let Some(cursor) = self.cursors.0.get(&selected_tool) {
            cursor.draw();
        }
    }

    pub fn sync(
        &mut self,
        main_color: [u8; 4],
        num_layers: usize,
        active_layer: usize,
        layers_vis: Vec<bool>,
        layers_alpha: Vec<u8>,
        preview_imgs: Vec<macroquad::prelude::Image>,
        palette: Vec<[u8; 4]>,
        mouse_canvas: Position<i32>,
        is_on_canvas: bool,
        selected_tool: Tool,
        visible_pixel_on_mouse: Option<[u8; 4]>,
    ) {
        self.toolbar.sync(main_color);
        self.layers_panel
            .sync(num_layers, active_layer, layers_vis, layers_alpha);
        self.preview.sync(preview_imgs);
        self.palette.sync(palette);
        self.status_bar.sync(
            mouse_canvas,
            is_on_canvas,
            selected_tool,
            visible_pixel_on_mouse,
        );
        self.menu.sync();
    }

    pub fn update(&mut self) -> Vec<Effect> {
        let mut events = Vec::new();

        let widget_color = egui::Color32::from_rgb(150, 150, 150);
        let widget_weak_color = egui::Color32::from_rgb(150, 150, 150);
        let bg_color = egui::Color32::from_rgb(175, 175, 175);
        let bg_strong_color = egui::Color32::from_rgb(230, 230, 230);
        let bg_weak_color = egui::Color32::from_rgb(150, 150, 150);
        let text_color = Some(egui::Color32::from_rgb(0, 0, 0));

        egui_macroquad::ui(|egui_ctx| {
            let mut visuals = egui_ctx.style().visuals.clone();
            visuals.dark_mode = false;
            visuals.menu_rounding = 2.0.into();
            visuals.window_rounding = 2.0.into();
            visuals.widgets.noninteractive.bg_fill = bg_color;
            visuals.widgets.noninteractive.weak_bg_fill = bg_weak_color;
            visuals.widgets.active.bg_fill = widget_color;
            visuals.widgets.active.weak_bg_fill = widget_weak_color;
            visuals.widgets.inactive.bg_fill = widget_color;
            visuals.widgets.inactive.weak_bg_fill = widget_weak_color;
            visuals.widgets.hovered.bg_fill = widget_color;
            visuals.widgets.hovered.weak_bg_fill = widget_weak_color;
            visuals.faint_bg_color = bg_weak_color;
            visuals.extreme_bg_color = bg_strong_color;
            visuals.panel_fill = bg_color;
            visuals.window_fill = bg_color;
            visuals.override_text_color = text_color;
            egui_ctx.set_visuals(visuals);

            let mut canvas_panel_events = self.update_canvas_panel(egui_ctx);
            events.append(&mut canvas_panel_events);

            let mut palette_events = self.palette.update(egui_ctx);
            events.append(&mut palette_events);

            let mut toolbar_events = self.toolbar.update(egui_ctx);
            events.append(&mut toolbar_events);

            let mut layers_events = self.layers_panel.update(egui_ctx);
            events.append(&mut layers_events);

            let mut menu_events = self.menu.update(egui_ctx);
            events.append(&mut menu_events);

            self.preview.update(egui_ctx);
            self.status_bar.update(egui_ctx);
            egui_ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::None);
        });

        if !events.is_empty() {
            events.push(UiEvent::GuiInteraction.into());
        }

        events
    }

    fn update_canvas_panel(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = Vec::new();

        egui::Window::new("Canvas")
            .default_pos((15., 30.))
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    let label = ui.label("w:");
                    ui.add(
                        egui::widgets::TextEdit::singleline(&mut self.canvas_size.0)
                            .desired_width(30.0),
                    )
                    .labelled_by(label.id);
                    let label = ui.label("h:");
                    ui.add(
                        egui::widgets::TextEdit::singleline(&mut self.canvas_size.1)
                            .desired_width(30.0),
                    )
                    .labelled_by(label.id);

                    if ui.button("resize").clicked() {
                        if let (Ok(w), Ok(h)) =
                            (self.canvas_size.0.parse(), self.canvas_size.1.parse())
                        {
                            events.push(Event::ResizeCanvas((w, h).into()).into());
                        }
                    }
                });

                ui.heading("Spritesheet");
                ui.horizontal(|ui| {
                    let label = ui.label("cols:");
                    ui.add(
                        egui::widgets::TextEdit::singleline(&mut self.spritesheet.0)
                            .desired_width(30.0),
                    )
                    .labelled_by(label.id);
                    let label = ui.label("rows:");
                    ui.add(
                        egui::widgets::TextEdit::singleline(&mut self.spritesheet.1)
                            .desired_width(30.0),
                    )
                    .labelled_by(label.id);
                    let btn = ui.button("Ok");
                    if btn.clicked() {
                        if let (Ok(w), Ok(h)) =
                            (self.spritesheet.0.parse(), self.spritesheet.1.parse())
                        {
                            events.push(Event::SetSpritesheet((w, h).into()).into());
                        }
                    }
                });
            });

        if egui_ctx.is_pointer_over_area() {
            events.push(Effect::UiEvent(UiEvent::MouseOverGui));
        }

        events
    }
}

pub struct CursorSet(HashMap<Tool, ToolCursor>);

impl CursorSet {
    pub fn new() -> Self {
        let tools = [
            (Tool::Brush, (0., -16.).into()),
            (Tool::Bucket, (0., -13.).into()),
            (Tool::Eraser, (0., -16.).into()),
            (Tool::Eyedropper, (0., -16.).into()),
            (Tool::Line, (0., -16.).into()),
            (Tool::Selection, (0., 0.).into()),
            (Tool::Move, (0., -16.).into()),
            (Tool::Rectangle, (0., -16.).into()),
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
        let texture = Texture2D::from_file_with_format(bytes, None);

        Self { texture, offset }
    }

    pub fn draw(&self) {
        let (x, y) = mouse_position();
        draw_texture_helper(
            self.texture,
            (x + self.offset.x, y + self.offset.y).into(),
            1.,
        )
    }
}
