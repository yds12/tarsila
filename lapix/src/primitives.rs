use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::{Add, Sub};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T: Number> {
    pub x: T,
    pub y: T,
}
pub type Position<T> = Point<T>;
pub type Size<T> = Point<T>;

impl<T: Number> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T: Number> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect<T: Number> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T: Number> Rect<T> {
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(self, x: T, y: T) -> bool {
        self.x <= x && self.x + self.w >= x && self.y <= y && self.y + self.h >= y
    }
}

impl<T: Number + Ord> Rect<T> {
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
