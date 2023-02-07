use crate::wrapped_image::WrappedImage;
use crate::UiState;
use lapix::graphics;
use lapix::{Bitmap, FreeImage, Point, Position, Rect, Selection, Size};
use macroquad::prelude::Color as MqColor;
use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

const DASHED_LINE_SEGMENT: f32 = 5.;
const DASHED_LINE_ANIMATION_MS: u128 = 250;
const SPRITESHEET_LINE_THICKNESS: f32 = 1.;
const SPRITESHEET_LINE_COLOR: MqColor = BLACK;

#[derive(Debug, Copy, Clone)]
pub struct DrawContext {
    pub spritesheet: Size<u8>,
    pub scale: f32,
    pub canvas_pos: Position<f32>,
    pub camera: Position<f32>,
    pub canvas_size: Size<f32>,
    pub selection: Option<Selection>,
}

pub fn draw_animated_dashed_line(p1: Point<i32>, p2: Point<i32>) {
    let len = graphics::distance(p1, p2);
    let (dist_x, dist_y) = ((p2.x - p1.x) as f32, (p2.y - p1.y) as f32);
    let segments = len / DASHED_LINE_SEGMENT;
    let (dx, dy) = (dist_x / segments, dist_y / segments);

    let iteration = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        / DASHED_LINE_ANIMATION_MS
        % 2) as usize;

    for i in 0..(segments as usize) {
        if i % 2 == iteration {
            draw_line(
                p1.x as f32 + i as f32 * dx as f32,
                p1.y as f32 + i as f32 * dy as f32,
                p1.x as f32 + (i as f32 + 1.) * dx as f32,
                p1.y as f32 + (i as f32 + 1.) * dy as f32,
                1.,
                BLACK,
            );
        };
    }
}

pub fn draw_animated_dashed_rect(rect: Rect<i32>) {
    draw_animated_dashed_line(
        Point::new(rect.x, rect.y),
        Point::new(rect.x + rect.w, rect.y),
    );
    draw_animated_dashed_line(
        Point::new(rect.x, rect.y),
        Point::new(rect.x, rect.y + rect.h),
    );
    draw_animated_dashed_line(
        Point::new(rect.x + rect.w, rect.y),
        Point::new(rect.x + rect.w, rect.y + rect.h),
    );
    draw_animated_dashed_line(
        Point::new(rect.x, rect.y + rect.h),
        Point::new(rect.x + rect.w, rect.y + rect.h),
    );
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

pub fn draw_canvas_bg(ctx: DrawContext) {
    let x = ctx.canvas_pos.x - ctx.camera.x;
    let y = ctx.canvas_pos.y - ctx.camera.y;
    let w = ctx.canvas_size.x * ctx.scale;
    let h = ctx.canvas_size.y * ctx.scale;

    let side = 4. * ctx.scale;

    // TODO: optimize this by storing a rendered texture that contains all
    // BG rectangles
    let bg1 = MqColor::new(0.875, 0.875, 0.875, 1.);
    let bg2 = MqColor::new(0.75, 0.75, 0.75, 1.);
    for i in 0..(w / side + 1.) as usize {
        for j in 0..(h / side + 1.) as usize {
            let cur_w = i as f32 * side;
            let cur_h = j as f32 * side;
            let next_w = (i + 1) as f32 * side;
            let next_h = (j + 1) as f32 * side;
            let x = x + i as f32 * side;
            let y = y + j as f32 * side;
            let w = if next_w <= w { side } else { w - cur_w };
            let h = if next_h <= h { side } else { h - cur_h };
            let color = if (i + j) % 2 == 0 { bg1 } else { bg2 };
            macroquad::prelude::draw_rectangle(x, y, w, h, color);
        }
    }
}

pub fn draw_selection(ctx: DrawContext, free_image: Option<&FreeImage<WrappedImage>>) {
    let rect = match ctx.selection {
        Some(Selection::FreeImage) => free_image.unwrap().rect,
        Some(Selection::Canvas(rect)) => rect.into(),
        _ => return,
    };

    let x0 = ctx.canvas_pos.x - ctx.camera.x;
    let y0 = ctx.canvas_pos.y - ctx.camera.y;
    let r = Rect {
        x: (x0 + rect.x as f32 * ctx.scale) as i32,
        y: (y0 + rect.y as f32 * ctx.scale) as i32,
        w: (rect.w as f32 * ctx.scale) as i32,
        h: (rect.h as f32 * ctx.scale) as i32,
    };
    draw_animated_dashed_rect(r);
}

pub fn draw_spritesheet_boundaries(ctx: DrawContext) {
    for i in 0..ctx.spritesheet.x {
        for j in 0..ctx.spritesheet.y {
            let x0 = ctx.canvas_pos.x - ctx.camera.x;
            let y0 = ctx.canvas_pos.y - ctx.camera.y;
            let w = ctx.canvas_size.x / ctx.spritesheet.x as f32 * ctx.scale;
            let h = ctx.canvas_size.y / ctx.spritesheet.y as f32 * ctx.scale;
            let x = x0 + i as f32 * w;
            let y = y0 + j as f32 * h;

            macroquad::prelude::draw_rectangle_lines(
                x,
                y,
                w,
                h,
                SPRITESHEET_LINE_THICKNESS,
                SPRITESHEET_LINE_COLOR,
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
        let w = texture.width();
        let h = texture.height();

        let x = state.canvas_pos().x - state.camera().x;
        let y = state.canvas_pos().y - state.camera().y;
        let scale = state.zoom();

        let params = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: w * scale,
                y: h * scale,
            }),
            ..Default::default()
        };

        let color = [255, 255, 255, state.layer(i).opacity()];
        macroquad::prelude::draw_texture_ex(texture, x, y, color.into(), params);
    }
}
