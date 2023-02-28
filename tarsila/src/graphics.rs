use crate::wrapped_image::WrappedImage;
use crate::UiState;
use lapix::graphics;
use lapix::{Bitmap, FreeImage, Point, Position, Rect, Selection, Size};
use macroquad::prelude::Color as MqColor;
use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

const DASHED_LINE_SEGMENT: f32 = 5.;
const DASHED_LINE_ANIMATION_MS: u128 = 250;
const SPRSHEET_LINE_THICK: f32 = 1.;
const SPRSHEET_LINE_COLOR: MqColor = BLACK;

#[derive(Debug, Copy, Clone)]
pub struct DrawContext {
    pub spritesheet: Size<u8>,
    pub scale: f32,
    pub canvas_pos: Position<f32>,
    pub camera: Position<f32>,
    pub canvas_size: Size<f32>,
    pub selection: Option<Selection>,
}

pub fn draw_texture_helper(texture: Texture2D, p: Position<f32>, scale: f32) {
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

pub fn draw_animated_dashed_line(p1: Point<i32>, p2: Point<i32>) {
    let len = graphics::distance(p1, p2);
    let dist: Point<f32> = (p2 - p1).into();
    let segments = len / DASHED_LINE_SEGMENT;
    let (dx, dy) = (dist.x / segments, dist.y / segments);

    let iteration = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        / DASHED_LINE_ANIMATION_MS
        % 2) as usize;

    for i in 0..(segments as usize) {
        let color = if i % 2 == iteration { BLACK } else { WHITE };
        draw_line(
            p1.x as f32 + i as f32 * dx,
            p1.y as f32 + i as f32 * dy,
            p1.x as f32 + (i as f32 + 1.) * dx,
            p1.y as f32 + (i as f32 + 1.) * dy,
            1.,
            color,
        );
    }
}

pub fn draw_animated_dashed_rect(rect: Rect<i32>) {
    draw_animated_dashed_line(rect.pos(), rect.top_right());
    draw_animated_dashed_line(rect.pos(), rect.bottom_left());
    draw_animated_dashed_line(rect.top_right(), rect.pos() + rect.size());
    draw_animated_dashed_line(rect.bottom_left(), rect.pos() + rect.size());
}

pub fn draw_free_image(
    ctx: DrawContext,
    img: &FreeImage<WrappedImage>,
    layer_opacity: u8,
    free_image_tex: Texture2D,
) {
    let w = img.texture.width() as f32;
    let h = img.texture.height() as f32;

    let x = ctx.canvas_pos.x - ctx.camera.x + img.rect.x as f32 * ctx.scale;
    let y = ctx.canvas_pos.y - ctx.camera.y + img.rect.y as f32 * ctx.scale;

    let params = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: w * ctx.scale,
            y: h * ctx.scale,
        }),
        ..Default::default()
    };

    let color = [255, 255, 255, layer_opacity];
    macroquad::prelude::draw_texture_ex(free_image_tex, x, y, color.into(), params);
}

pub fn draw_selection(ctx: DrawContext, free_image: Option<&FreeImage<WrappedImage>>) {
    let rect = match ctx.selection {
        Some(Selection::FreeImage) => free_image.unwrap().rect,
        Some(Selection::Canvas(rect)) => rect,
        _ => return,
    };

    let p0 = ctx.canvas_pos - ctx.camera;
    let r = Rect {
        x: (p0.x + rect.x as f32 * ctx.scale) as i32,
        y: (p0.y + rect.y as f32 * ctx.scale) as i32,
        w: (rect.w as f32 * ctx.scale) as i32,
        h: (rect.h as f32 * ctx.scale) as i32,
    };
    draw_animated_dashed_rect(r);
}

pub fn draw_spritesheet_boundaries(ctx: DrawContext) {
    for i in 0..ctx.spritesheet.x {
        for j in 0..ctx.spritesheet.y {
            let p0 = ctx.canvas_pos - ctx.camera;
            let w = ctx.canvas_size.x / ctx.spritesheet.x as f32 * ctx.scale;
            let h = ctx.canvas_size.y / ctx.spritesheet.y as f32 * ctx.scale;
            let x = p0.x + i as f32 * w;
            let y = p0.y + j as f32 * h;

            macroquad::prelude::draw_rectangle_lines(
                x,
                y,
                w,
                h,
                SPRSHEET_LINE_THICK,
                SPRSHEET_LINE_COLOR,
            );
        }
    }
}

pub fn draw_canvas(state: &UiState) {
    for i in 0..state.num_layers() {
        if !state.layer(i).visible() {
            continue;
        }

        let texture = state.layer_tex(i);
        let size = Size::new(texture.width(), texture.height());
        let p = state.canvas_pos() - state.camera();
        let scale = state.zoom();

        let params = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: size.x * scale,
                y: size.y * scale,
            }),
            ..Default::default()
        };

        let color = [255, 255, 255, state.layer(i).opacity()];
        macroquad::prelude::draw_texture_ex(texture, p.x, p.y, color.into(), params);
    }
}
