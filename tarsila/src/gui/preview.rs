use crate::wrapped_image::WrappedImage;
use lapix::{Bitmap, Size};
use std::time::{SystemTime, UNIX_EPOCH};

const MS_PER_FRAME: usize = 100;

pub struct Preview {
    spritesheet: Size<u8>,
    image: Option<WrappedImage>,
    egui_image: Option<egui::ColorImage>,
    texture: Option<egui::TextureHandle>,
    scale: String,
}

impl Preview {
    pub fn new() -> Self {
        Self {
            spritesheet: (1, 1).into(),
            image: None,
            egui_image: None,
            texture: None,
            scale: "1".to_owned(),
        }
    }

    pub fn sync(&mut self, spritesheet: Size<u8>, image: Option<WrappedImage>) {
        self.spritesheet = spritesheet;

        if let Some(image) = image {
            self.texture = None;
            self.egui_image = None;

            let w = image.width() as usize;
            let h = image.height() as usize;
            let img = egui::ColorImage::from_rgba_unmultiplied([w, h], &image.bytes());
            self.egui_image = Some(img);
            self.image = Some(image);
        }
    }

    pub fn update(&mut self, egui_ctx: &egui::Context) {
        egui::Window::new("Preview")
            .scroll2([true, true])
            .anchor(egui::Align2::RIGHT_BOTTOM, (-15., -15.))
            .show(egui_ctx, |ui| {
                let label = ui.label("scale:");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.scale).desired_width(30.0))
                    .labelled_by(label.id);

                let len = self.spritesheet.x as usize * self.spritesheet.y as usize;
                let t = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let i = (t as usize / MS_PER_FRAME) % len;

                if let Some(image) = &self.egui_image {
                    let uv = self.get_spritesheet_rect(i);
                    let frame_ratios = self.frame_ratios();
                    let tex: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                        ui.ctx()
                            .load_texture("", image.clone(), egui::TextureOptions::NEAREST)
                    });
                    let frame_size = frame_ratios * tex.size_vec2();
                    let scale = self.scale.parse().unwrap_or(1.);
                    let image = egui::Image::new(tex, frame_size * scale)
                        .bg_fill(egui::Color32::LIGHT_GRAY)
                        .uv(uv);
                    ui.add(image);
                }
            });
    }

    fn frame_ratios(&self) -> egui::Vec2 {
        let nx = self.spritesheet.x;
        let ny = self.spritesheet.y;

        egui::Vec2::new(1. / nx as f32, 1. / ny as f32)
    }

    fn get_spritesheet_rect(&self, iteration: usize) -> egui::Rect {
        let nx = self.spritesheet.x;
        let ny = self.spritesheet.y;
        let frame_ratios = self.frame_ratios();

        let x = ((iteration % (nx as usize)) as f32) / (nx as f32);
        let y = ((iteration / (nx as usize)) as f32) / (ny as f32);

        egui::Rect {
            min: (x, y).into(),
            max: (x + frame_ratios.x, y + frame_ratios.y).into(),
        }
    }
}
