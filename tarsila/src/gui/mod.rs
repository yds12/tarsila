use crate::wrapped_image::WrappedImage;
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

pub struct Gui {
    toolbar: Toolbar,
    layers_panel: LayersPanel,
    canvas_size: (String, String),
    last_file: Option<PathBuf>,
    spritesheet: (String, String),
    preview: Preview,
    palette: Palette,
    status_bar: StatusBar,
    menu: MenuBar,
    mouse_on_canvas: bool,
}

impl Gui {
    pub fn new(canvas_size: Size<u16>) -> Self {
        Self {
            toolbar: Toolbar::new(),
            layers_panel: LayersPanel::new(),
            canvas_size: (canvas_size.x.to_string(), canvas_size.y.to_string()),
            last_file: None,
            spritesheet: ("1".to_owned(), "1".to_owned()),
            preview: Preview::new(),
            palette: Palette::new(),
            status_bar: StatusBar::new(),
            menu: MenuBar::new(),
            mouse_on_canvas: false,
        }
    }

    pub fn sync(
        &mut self,
        main_color: [u8; 4],
        num_layers: usize,
        active_layer: usize,
        layers_vis: Vec<bool>,
        layers_alpha: Vec<u8>,
        preview_img: Option<WrappedImage>,
        palette: Vec<[u8; 4]>,
        mouse_canvas: Position<i32>,
        is_on_canvas: bool,
        selected_tool: Tool,
        visible_pixel_on_mouse: Option<[u8; 4]>,
        canvas_size: Size<i32>,
        spritesheet: Size<u8>,
        zoom: f32,
        fps: f32,
    ) {
        self.mouse_on_canvas = is_on_canvas;

        self.toolbar.sync(main_color);

        self.layers_panel
            .sync(num_layers, active_layer, layers_vis, layers_alpha);

        self.preview.sync(spritesheet, preview_img);

        self.palette.sync(palette);

        self.status_bar.sync(
            mouse_canvas,
            is_on_canvas,
            selected_tool,
            visible_pixel_on_mouse,
            canvas_size,
            zoom,
            fps,
        );

        self.menu.sync(canvas_size, spritesheet);
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

            if self.mouse_on_canvas {
                egui_ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::None);
            }
        });

        if !events.is_empty() {
            events.push(UiEvent::GuiInteraction.into());
        }

        events
    }

    fn update_canvas_panel(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = Vec::new();

        if egui_ctx.is_pointer_over_area() {
            events.push(Effect::UiEvent(UiEvent::MouseOverGui));
        }

        events
    }
}
