use lapix::graphics;
use lapix::{Point, Rect};
use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

const DASHED_LINE_SEGMENT: f32 = 5.;
const DASHED_LINE_ANIMATION_MS: u128 = 250;

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
