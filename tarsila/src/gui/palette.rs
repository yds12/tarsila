use crate::wrapped_image::WrappedImage;
use crate::Effect;
use lapix::{Bitmap, Event};
use macroquad::prelude::Image as MqImage;

const BTN_SIZE: i32 = 20;

pub struct Palette {
    colors: Vec<[u8; 4]>,
    images: Vec<MqImage>,
    egui_images: Vec<egui::ColorImage>,
    textures: Vec<Option<egui::TextureHandle>>,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            images: Vec::new(),
            egui_images: Vec::new(),
            textures: Vec::new(),
        }
    }

    // TODO: this is a copy and paste of the sync fn in `Preview`, DRY
    pub fn sync(&mut self, colors: Vec<[u8; 4]>) {
        if !colors.is_empty() {
            self.colors = colors;
            self.images = self
                .colors
                .iter()
                .map(|c| WrappedImage::new((BTN_SIZE, BTN_SIZE).into(), (*c).into()).0)
                .collect();
            self.textures = (0..self.images.len()).map(|_| None).collect();
            self.egui_images = Vec::new();

            for image in &self.images {
                let w = image.width();
                let h = image.height();
                let img = egui::ColorImage::from_rgba_unmultiplied([w, h], &image.bytes);
                self.egui_images.push(img);
            }
        }
    }

    pub fn update(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut fx = Vec::new();

        egui::Window::new("Palette").show(egui_ctx, |ui| {
            let btn = ui.button("Load");
            if btn.clicked() {
                let dialog = rfd::FileDialog::new();

                if let Some(path) = dialog.pick_file() {
                    fx.push(Event::LoadPalette(path).into());
                }
            }
            ui.horizontal_wrapped(|ui| {
                ui.set_max_width(160.);
                ui.spacing_mut().item_spacing = egui::vec2(0., 0.);
                ui.spacing_mut().button_padding = egui::vec2(1., 1.);

                for i in 0..self.textures.len() {
                    let tex = &mut self.textures[i];
                    let image = &mut self.egui_images[i];
                    let tex: &egui::TextureHandle = tex.get_or_insert_with(|| {
                        ui.ctx().load_texture("", image.clone(), Default::default())
                    });
                    let tooltip = format!(
                        "Select color {:?} (left click to remove from palette",
                        self.colors[i]
                    );

                    let btn = egui::ImageButton::new(tex, tex.size_vec2());
                    let btn = ui.add(btn).on_hover_text(tooltip);
                    if btn.clicked() {
                        fx.push(Event::SetMainColor(self.colors[i].into()).into());
                    }
                    if btn.clicked_by(egui::PointerButton::Secondary) {
                        fx.push(Event::RemoveFromPalette(self.colors[i].into()).into());
                    }
                }
            });
        });

        fx
    }
}
