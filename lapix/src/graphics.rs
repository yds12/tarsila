//! Functions to calculate graphics like lines, rectangles, etc. in a discrete
//! 2D space

use std::collections::HashSet;

use crate::{Point, Rect};

/// Get the distance between two [`Point`]s
pub fn distance(p1: Point<i32>, p2: Point<i32>) -> f32 {
    ((((p1.x - p2.x) as i64).pow(2) + ((p1.y - p2.y) as i64).pow(2)) as f64).sqrt() as f32
}

/// Get the set of [`Point`]s needed to draw a line between two points
pub fn line(p1: Point<i32>, p2: Point<i32>) -> Vec<Point<i32>> {
    let mut line = Vec::new();
    let diff = p2 - p1;
    let dist = distance(p1, p2);
    let dx = if dist < 0.1 { 0. } else { diff.x as f32 / dist };
    let dy = if dist < 0.1 { 0. } else { diff.y as f32 / dist };

    for i in 0..=dist.round() as usize {
        let x = (p1.x as f32 + (i as f32 * dx)).round() as i32;
        let y = (p1.y as f32 + (i as f32 * dy)).round() as i32;

        if let Some(Point { x: x0, y: y0 }) = line.last() {
            if x == *x0 && y == *y0 {
                continue;
            }
        }

        line.push((x, y).into());
    }

    line
}

/// Get the set of [`Point`]s needed to draw a rectangle between two points
pub fn rectangle(p1: Point<i32>, p2: Point<i32>) -> Vec<Point<i32>> {
    let l1 = line((p1.x, p1.y).into(), (p1.x, p2.y).into());
    let l2 = line((p1.x, p1.y).into(), (p2.x, p1.y).into());
    let l3 = line((p2.x, p1.y).into(), (p2.x, p2.y).into());
    let l4 = line((p1.x, p2.y).into(), (p2.x, p2.y).into());

    vec![l1, l2, l3, l4].into_iter().flatten().collect()
}

/// Get the set of [`Point`]s needed to draw an ellipse between two points
/// TODO there are still some imperfections here
pub fn ellipse(p1: Point<i32>, p2: Point<i32>) -> Vec<Point<i32>> {
    let a = (p1.x - p2.x).abs() as f32 / 2.0;
    let b = (p1.y - p2.y).abs() as f32 / 2.0;

    let low_x = std::cmp::min(p1.x, p2.x);
    let low_y = std::cmp::min(p1.y, p2.y);
    let high_x = std::cmp::max(p1.x, p2.x);
    let high_y = std::cmp::max(p1.y, p2.y);
    let bounds = Rect::new(low_x, low_y, high_x - low_x, high_y - low_y);
    let xspan = ((p1.x - p2.x).abs() as f32 / 2.0).round() as i32;
    let yspan = ((p1.y - p2.y).abs() as f32 / 2.0).round() as i32;

    let mut points = HashSet::new();

    let sampling_level = yspan;
    let step = 1. / sampling_level as f32;

    // For each x, we'll find the corresponding y values
    for x in 0..(xspan) {
        for delta in 0..sampling_level {
            let x = x as f32 + (delta as f32 * step);
            // Formula:
            // sqrt(((a2-x2)*b2)/a2)
            let inner = (a.powf(2.0) - x.powf(2.0)) * b.powf(2.0) / a.powf(2.0);
            let root = inner.sqrt();

            let ys = vec![root, -root];

            for y in ys {
                let xx = x.round() as i32 + low_x + xspan;
                let yy = y.round() as i32 + low_y + yspan;

                if bounds.contains(xx, yy) {
                    points.insert(Point::new(xx, yy));
                }

                let xx = -x.round() as i32 + low_x + xspan;
                let yy = y.round() as i32 + low_y + yspan;

                if bounds.contains(xx, yy) {
                    points.insert(Point::new(xx, yy));
                }
            }
        }
    }

    points.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case((0,0), (2,0), vec![(0, 0), (1, 0), (2, 0)])]
    #[test_case((0,0), (0,2), vec![(0, 0), (0, 1), (0, 2)])]
    #[test_case((0,0), (2,2), vec![(0, 0), (1, 1), (2, 2)])]
    #[test_case((2,0), (0,0), vec![(0, 0), (1, 0), (2, 0)])]
    #[test_case((0,0), (3,1), vec![(0, 0), (1, 0), (2, 1), (3, 1)])]
    fn simple_line_cases(p1: (i32, i32), p2: (i32, i32), expected: Vec<(i32, i32)>) {
        let mut l = line(p1.into(), p2.into());
        l.sort();

        assert_eq!(l, expected.into_iter().map(Into::into).collect::<Vec<_>>());
    }

    #[test]
    fn odd_lines() {
        let p1 = (0, 0);
        let p2 = (2, 1);
        let expect = vec![(0, 0), (2, 1)];
        let either = vec![(1, 0), (1, 1)];
        let l = line(p1.into(), p2.into());

        for expected in expect {
            assert!(l.contains(&expected.into()));
        }

        assert!(l.contains(&either[0].into()) || l.contains(&either[1].into()));
    }
}
