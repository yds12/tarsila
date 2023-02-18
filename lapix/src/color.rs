use serde::{Deserialize, Serialize};

pub const TRANSPARENT: Color = Color::new(0, 0, 0, 0);
pub const BLACK: Color = Color::new(0, 0, 0, 255);

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorF32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<ColorF32> for Color {
    fn from(value: ColorF32) -> Self {
        Self {
            r: (value.r * 255.) as u8,
            g: (value.g * 255.) as u8,
            b: (value.b * 255.) as u8,
            a: (value.a * 255.) as u8,
        }
    }
}

impl From<Color> for ColorF32 {
    fn from(value: Color) -> Self {
        Self {
            r: (value.r as f32 / 255.),
            g: (value.g as f32 / 255.),
            b: (value.b as f32 / 255.),
            a: (value.a as f32 / 255.),
        }
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        [value.r, value.g, value.b, value.a]
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
            a: value.3,
        }
    }
}

impl From<(f32, f32, f32, f32)> for ColorF32 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
            a: value.3,
        }
    }
}

impl ColorF32 {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Blend this color on top of another
    pub fn blend_over(&self, other: Self) -> Self {
        let fg = ColorF32::from(*self);
        let bg = ColorF32::from(other);

        ColorF32::new(
            (fg.r * fg.a) + (bg.r * bg.a * (1. - fg.a)),
            (fg.g * fg.a) + (bg.g * bg.a * (1. - fg.a)),
            (fg.b * fg.a) + (bg.b * bg.a * (1. - fg.a)),
            fg.a + bg.a * (1. - fg.a),
        )
        .into()
    }

    pub fn with_multiplied_alpha(&self, alpha: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (((self.a as f32 / 255.) * (alpha as f32 / 255.)) * 255.) as u8,
        }
    }
}
