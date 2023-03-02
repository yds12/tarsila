use crate::wrapped_image::WrappedImage;
use lapix::{Bitmap, Size};
use std::time::{SystemTime, UNIX_EPOCH};

const MS_PER_FRAME: usize = 100;

pub struct Preview {
    spritesheet: Size<u8>,
    images: Vec<egui::ColorImage>,
    textures: Vec<Option<egui::TextureHandle>>,
    scale: String,
}

impl Preview {
    pub fn new() -> Self {
        Self {
            spritesheet: (1, 1).into(),
            images: Vec::new(),
            textures: Vec::new(),
            scale: "1".to_owned(),
        }
    }

    pub fn sync(&mut self, spritesheet: Size<u8>, images: Option<Vec<WrappedImage>>) {
        self.spritesheet = spritesheet;

        if let Some(imgs) = images {
            self.textures = (0..imgs.len()).map(|_| None).collect();
            self.images = Vec::new();

            for image in imgs {
                let w = image.width() as usize;
                let h = image.height() as usize;
                let img = egui::ColorImage::from_rgba_unmultiplied([w, h], &image.bytes());
                self.images.push(img);
            }
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
                let frame = (t as usize / MS_PER_FRAME) % len;
                let uv = self.get_spritesheet_rect(frame);
                let frame_ratios = self.frame_ratios();

                let mut rect = None;
                for i in 0..self.images.len() {
                    let tex: &egui::TextureHandle = self.textures[i].get_or_insert_with(|| {
                        ui.ctx().load_texture(
                            "",
                            self.images[i].clone(),
                            egui::TextureOptions::NEAREST,
                        )
                    });
                    let frame_size = frame_ratios * tex.size_vec2();
                    let scale = self.scale.parse().unwrap_or(1.);

                    match rect {
                        None => {
                            let image = egui::Image::new(tex, frame_size * scale)
                                .bg_fill(egui::Color32::LIGHT_GRAY)
                                .uv(uv);
                            rect = Some(ui.add(image).rect);
                        }
                        Some(r) => {
                            let image2 = egui::Image::new(tex, frame_size * scale).uv(uv);
                            ui.put(r, image2);
                        }
                    }
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
