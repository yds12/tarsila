//! Basic types for numbers, points, rectangles, etc.

use serde::{Deserialize, Serialize};
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::{Add, Sub};

/// Represents a number
pub trait Number:
    Add<Output = Self> + Sub<Output = Self> + PartialOrd<Self> + Copy + Debug + Clone + Sized
{
}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for isize {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for usize {}
impl Number for f32 {}
impl Number for f64 {}

/// Represents a 2D point.
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Point<T: Number> {
    pub x: T,
    pub y: T,
}
/// Represents a 2D position
pub type Position<T> = Point<T>;
/// Represents a 2D size
pub type Size<T> = Point<T>;

impl<T: Number> Sub for Point<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Number> Add for Point<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Number> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Point<i32>> for Point<f32> {
    fn from(value: Point<i32>) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl<T: Number> Point<T> {
    /// Create a new point
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<f32> {
    /// The point (0, 0)
    pub const ZERO_F32: Self = Point::new(0., 0.);
    /// The point (1, 1)
    pub const ONE_F32: Self = Point::new(1., 1.);
}

impl Point<i32> {
    /// The point (0, 0)
    pub const ZERO: Self = Point::new(0, 0);
    /// The point (1, 1)
    pub const ONE: Self = Point::new(1, 1);

    /// The absolute difference in each coordinate between this and another
    /// point.
    pub fn abs_diff(&self, p: Self) -> Self {
        Self {
            x: (self.x - p.x).abs(),
            y: (self.y - p.y).abs(),
        }
    }

    /// Get the top-left corner of a rectangle determined by this and another
    /// point
    pub fn rect_min_corner(&self, p: Self) -> Self {
        Self {
            x: std::cmp::min(self.x, p.x),
            y: std::cmp::min(self.y, p.y),
        }
    }
}

/// Represents one of the 4 basic 2D directions
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Represents a rectangle by its top left coordinate plus width and height
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect<T: Number> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T: Number> Rect<T> {
    /// Create a new rectangle
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }

    /// Check whether this rectangle contains a certain point
    pub fn contains(self, x: T, y: T) -> bool {
        self.x <= x && self.x + self.w >= x && self.y <= y && self.y + self.h >= y
    }

    /// Get the position of the rectangle
    pub fn pos(self) -> Position<T> {
        Position {
            x: self.x,
            y: self.y,
        }
    }

    /// Get the size of the rectangle
    pub fn size(self) -> Size<T> {
        Size {
            x: self.w,
            y: self.h,
        }
    }

    /// Get the top right corner of the rectangle
    pub fn top_right(self) -> Point<T> {
        Point {
            x: self.x + self.w,
            y: self.y,
        }
    }

    /// Get the bottom left corner of the rectangle
    pub fn bottom_left(self) -> Point<T> {
        Point {
            x: self.x,
            y: self.y + self.h,
        }
    }
}

impl<T: Number + Ord> Rect<T> {
    /// Clamp the rectangle to the bounds of another (intersection)
    pub fn clip_to(self, other: Self) -> Self {
        let x = self.x.clamp(other.x, other.x + other.w);
        let y = self.y.clamp(other.y, other.y + other.h);
        let x1 = (self.x + self.w).clamp(other.x, other.x + other.w);
        let y1 = (self.y + self.h).clamp(other.y, other.y + other.h);

        Self {
            x,
            y,
            w: x1 - x,
            h: y1 - y,
        }
    }
}

impl From<Rect<u16>> for Rect<i32> {
    fn from(val: Rect<u16>) -> Self {
        Self {
            x: val.x as i32,
            y: val.y as i32,
            w: val.w as i32,
            h: val.h as i32,
        }
    }
}

impl From<Rect<i32>> for Rect<u16> {
    fn from(val: Rect<i32>) -> Self {
        assert!(val.x >= 0);
        assert!(val.y >= 0);

        Self {
            x: val.x as u16,
            y: val.y as u16,
            w: val.w as u16,
            h: val.h as u16,
        }
    }
}

impl From<Rect<f32>> for Rect<i32> {
    fn from(val: Rect<f32>) -> Self {
        Self {
            x: val.x as i32,
            y: val.y as i32,
            w: val.w as i32,
            h: val.h as i32,
        }
    }
}

impl From<Rect<i32>> for Rect<f32> {
    fn from(val: Rect<i32>) -> Self {
        Self {
            x: val.x as f32,
            y: val.y as f32,
            w: val.w as f32,
            h: val.h as f32,
        }
    }
}

impl<T: Number> From<(T, T, T, T)> for Rect<T> {
    fn from(val: (T, T, T, T)) -> Self {
        Self {
            x: val.0,
            y: val.1,
            w: val.2,
            h: val.3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case((0, 0, 10, 10), (2, 2, 4, 4), (2, 2, 4, 4))]
    #[test_case((-1, 0, 1, 1), (-1, 0, 1, 1), (-1, 0, 1, 1))]
    #[test_case((-1, 0, 1, 1), (-2, 0, 2, 1), (-1, 0, 1, 1))]
    #[test_case((0, 0, 2, 2), (-2, -2, 4, 4), (0, 0, 2, 2))]
    #[test_case((0, 0, 2, 2), (0, 0, 1, 1), (0, 0, 1, 1))]
    #[test_case((0, 0, 2, 2), (-2, 0, 4, 1), (0, 0, 2, 1))]
    fn rect_clip<R: Into<Rect<i32>>>(r: R, clip: R, res: R) {
        assert_eq!(r.into().clip_to(clip.into()), res.into());
    }

    #[test_case((0, 0, 1, 1), (0, 0), true)]
    #[test_case((0, 0, 2, 2), (1, 1), true)]
    #[test_case((0, 0, 1, 1), (0, 1), true)]
    #[test_case((0, 0, 1, 1), (1, 0), true)]
    #[test_case((0, 0, 1, 1), (2, 0), false)]
    #[test_case((0, 0, 1, 1), (0, 2), false)]
    #[test_case((0, 0, 1, 1), (-1, 0), false)]
    fn rect_contains(r: impl Into<Rect<i32>>, p: (i32, i32), res: bool) {
        assert_eq!(r.into().contains(p.0, p.1), res);
    }

    #[test]
    fn rect_extremes() {
        let r = Rect::new(0, 1, 2, 3);
        assert_eq!(r.pos(), Position::new(0, 1));
        assert_eq!(r.size(), Size::new(2, 3));
        assert_eq!(r.top_right(), Position::new(2, 1));
        assert_eq!(r.bottom_left(), Position::new(0, 4));
    }
}
