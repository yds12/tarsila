use crate::graphics::DrawContext;
use crate::wrapped_image::WrappedImage;
use lapix::color::TRANSPARENT;
use lapix::{Bitmap, Color};
use macroquad::prelude::*;

const CHECKERED_TILE_SIZE: f32 = 64.;
const CHECKERED_TILE_PIX_PER_SQ: f32 = 4.;
const BG_COLOR_1: Color = Color::new(224, 224, 224, 255);
const BG_COLOR_2: Color = Color::new(192, 192, 192, 255);

pub struct Background {
    checkered_tile: Texture2D,
    tile_size: f32,
    pix_per_sq: f32,
}

impl Background {
    pub fn new() -> Self {
        let mut checkered_tile = WrappedImage::new(
            (CHECKERED_TILE_SIZE as i32, CHECKERED_TILE_SIZE as i32).into(),
            TRANSPARENT,
        );

        let bg1 = BG_COLOR_1;
        let bg2 = BG_COLOR_2;

        for i in 0..(CHECKERED_TILE_SIZE as i32) {
            for j in 0..(CHECKERED_TILE_SIZE as i32) {
                let color = if (i + j) % 2 == 0 { bg1 } else { bg2 };
                checkered_tile.set_pixel((i, j).into(), color);
            }
        }

        let checkered_tile = Texture2D::from_image(&checkered_tile.0);
        checkered_tile.set_filter(FilterMode::Nearest);

        Self {
            checkered_tile,
            tile_size: CHECKERED_TILE_SIZE,
            pix_per_sq: CHECKERED_TILE_PIX_PER_SQ,
        }
    }

    pub fn draw(&self, ctx: DrawContext) {
        let p = ctx.canvas_pos - ctx.camera;
        let side = self.tile_size * self.pix_per_sq;

        for i in 0..(ctx.canvas_size.x / side).ceil() as usize {
            for j in 0..(ctx.canvas_size.y / side).ceil() as usize {
                let pos = p + (i as f32 * side * ctx.scale, j as f32 * side * ctx.scale).into();

                let pixels_left = ctx.canvas_size - (i as f32 * side, j as f32 * side).into();
                let x_frac = pixels_left.x / side;
                let y_frac = pixels_left.y / side;

                let params = DrawTextureParams {
                    source: Some(Rect {
                        x: 0.,
                        y: 0.,
                        w: self.tile_size * x_frac,
                        h: self.tile_size * y_frac,
                    }),
                    dest_size: Some(vec2(side * x_frac * ctx.scale, side * y_frac * ctx.scale)),
                    ..Default::default()
                };
                draw_texture_ex(
                    self.checkered_tile,
                    pos.x,
                    pos.y,
                    macroquad::prelude::WHITE,
                    params,
                );
            }
        }
    }
}
