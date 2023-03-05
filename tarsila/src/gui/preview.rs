use crate::UiState;
use lapix::{Position, Rect, Size};
use std::time::{SystemTime, UNIX_EPOCH};

const MS_PER_FRAME: usize = 100;

pub struct Preview {
    spritesheet: Size<u8>,
    canvas_size: Size<i32>,
    image: egui::ColorImage,
    texture: Option<egui::TextureHandle>,
    scale: String,
    layers_vis: Vec<bool>,
    layers_alpha: Vec<u8>,
    config: Option<(Position<f32>, Rect<f32>)>,
}

impl Preview {
    pub fn new() -> Self {
        let bytes = [0, 0, 0, 0];
        Self {
            spritesheet: (1, 1).into(),
            canvas_size: (0, 0).into(),
            image: egui::ColorImage::from_rgba_unmultiplied([1, 1], &bytes),
            texture: None,
            scale: "1".to_owned(),
            layers_vis: Vec::new(),
            layers_alpha: Vec::new(),
            config: None,
        }
    }

    pub fn sync(
        &mut self,
        spritesheet: Size<u8>,
        canvas_size: Size<i32>,
        layers_vis: Vec<bool>,
        layers_alpha: Vec<u8>,
    ) {
        self.spritesheet = spritesheet;
        self.layers_vis = layers_vis;
        self.layers_alpha = layers_alpha;
        self.canvas_size = canvas_size;
    }

    pub fn update(&mut self, egui_ctx: &egui::Context) {
        egui::Window::new("Preview")
            .anchor(egui::Align2::RIGHT_BOTTOM, (-15., -15.))
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    let label = ui.label("scale:");
                    ui.add(
                        egui::widgets::TextEdit::singleline(&mut self.scale).desired_width(30.0),
                    )
                    .labelled_by(label.id);
                });
                let scroll_area = egui::ScrollArea::new([true, true]);
                scroll_area.show_viewport(ui, |ui, viewport| {
                    let frame_ratios = self.frame_ratios();

                    let tex: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                        ui.ctx()
                            .load_texture("", self.image.clone(), egui::TextureOptions::NEAREST)
                    });
                    let frame_size = frame_ratios
                        * egui::vec2(self.canvas_size.x as f32, self.canvas_size.y as f32);
                    let scale = self.scale.parse().unwrap_or(1.);

                    let image = egui::Image::new(tex, frame_size * scale)
                        .bg_fill(egui::Color32::LIGHT_GRAY);
                    let r = ui.add(image).rect;

                    let r: Rect<i32> =
                        Rect::new(r.min.x, r.min.y, r.max.x - r.min.x, r.max.y - r.min.y).into();
                    let cr = ui.clip_rect();
                    let clip: Rect<i32> =
                        Rect::new(cr.min.x, cr.min.y, cr.max.x - cr.min.x, cr.max.y - cr.min.y)
                            .into();
                    if clip.w >= 0 && clip.h >= 0 {
                        let r = r.clip_to(clip);
                        let offset = Position::new(viewport.min.x, viewport.min.y);
                        self.config = Some((offset, r.into()));
                    } else {
                        self.config = None;
                    }
                });
            });
    }

    // TODO this method has a lot in common with graphics::draw_canvas,
    // it would be better to unify these implementations
    pub fn draw(&self, state: &UiState) {
        use macroquad::prelude::*;

        if let Some((offset, rect)) = self.config {
            let frames = self.spritesheet.x as usize * self.spritesheet.y as usize;
            let t = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let frame = (t as usize / MS_PER_FRAME) % frames;
            let frame_size: Size<f32> = (
                (self.canvas_size.x as usize / self.spritesheet.x as usize) as f32,
                (self.canvas_size.y as usize / self.spritesheet.y as usize) as f32,
            )
                .into();
            let frame = Rect {
                x: ((frame % (self.spritesheet.x as usize)) as f32) * frame_size.x,
                y: ((frame / (self.spritesheet.x as usize)) as f32) * frame_size.y,
                w: frame_size.x,
                h: frame_size.y,
            };
            let preview_scale = self.scale.parse().unwrap_or(1.);
            let scrollarea_frame = Rect {
                x: frame.x + (offset.x / preview_scale),
                y: frame.y + (offset.y / preview_scale),
                w: rect.w / preview_scale,
                h: rect.h / preview_scale,
            };

            for i in 0..state.num_layers() {
                if !state.layer(i).visible() {
                    continue;
                }

                let texture = state.layer_tex(i);
                let params = DrawTextureParams {
                    source: Some(scrollarea_frame),
                    dest_size: Some(Vec2 {
                        x: rect.w,
                        y: rect.h,
                    }),
                    ..Default::default()
                };

                let color = [255, 255, 255, state.layer(i).opacity()];
                draw_texture_ex(texture, rect.x, rect.y, color.into(), params);
            }
        }
    }

    fn frame_ratios(&self) -> egui::Vec2 {
        let nx = self.spritesheet.x;
        let ny = self.spritesheet.y;

        egui::Vec2::new(1. / nx as f32, 1. / ny as f32)
    }
}
