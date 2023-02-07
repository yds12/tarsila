use macroquad::prelude::Image as MqImage;
use std::time::{SystemTime, UNIX_EPOCH};

const MS_PER_FRAME: usize = 100;

pub struct Preview {
    images: Vec<MqImage>,
    egui_images: Vec<egui::ColorImage>,
    textures: Vec<Option<egui::TextureHandle>>,
    scale: String,
}

impl Preview {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            egui_images: Vec::new(),
            textures: Vec::new(),
            scale: "1".to_owned(),
        }
    }

    pub fn sync(&mut self, images: Vec<MqImage>) {
        if !images.is_empty() {
            self.textures = (0..images.len()).map(|_| None).collect();
            self.images = images;
            self.egui_images = Vec::new();

            for image in &self.images {
                let w = image.width();
                let h = image.height();
                let img = egui::ColorImage::from_rgba_unmultiplied([w, h], &image.bytes);
                self.egui_images.push(img);
            }
        }
    }

    pub fn update(&mut self, egui_ctx: &egui::Context) {
        egui::Window::new("Preview")
            .anchor(egui::Align2::RIGHT_BOTTOM, (-15., -15.))
            .show(egui_ctx, |ui| {
                let label = ui.label("scale:");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.scale).desired_width(30.0))
                    .labelled_by(label.id);

                let len = self.egui_images.len();
                let t = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let i = (t as usize / MS_PER_FRAME) % len;

                if let Some(image) = self.egui_images.get(i) {
                    let tex: &egui::TextureHandle = self.textures[i].get_or_insert_with(|| {
                        // TODO: the NEAREST filter is not working (it's doing
                        // LINEAR) at the moment, but it seems that there is
                        // some work going on in egui-miniquad, so hopefully
                        // this will work soon
                        ui.ctx()
                            .load_texture("", image.clone(), egui::TextureOptions::NEAREST)
                    });
                    let scale = self.scale.parse().unwrap_or(1.);
                    ui.image(tex, tex.size_vec2() * scale);
                }
            });
    }
}
