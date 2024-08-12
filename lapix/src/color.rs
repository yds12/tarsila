//! Basic types for colors

use serde::{Deserialize, Serialize};

/// The color transparent
pub const TRANSPARENT: Color = Color::new(0, 0, 0, 0);
/// The color black
pub const BLACK: Color = Color::new(0, 0, 0, 255);

/// Represents an RGBA color, with component values from 0-255
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Represents an RGBA color, with component values from 0-1
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
    /// Create a new color
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn dist(&self, other: &Self) -> f32 {
        ((self.r - other.r).powf(2.)
            + (self.g - other.g).powf(2.)
            + (self.b - other.b).powf(2.)
            + (self.a - other.a).powf(2.))
        .sqrt()
    }
}

impl Color {
    /// Create a new color
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Blend this color on top of another
    pub fn blend_over(&self, other: Self) -> Self {
        let fg = ColorF32::from(*self);
        let bg = ColorF32::from(other);

        let res_alpha = fg.a + bg.a * (1. - fg.a);
        ColorF32::new(
            ((fg.r * fg.a) + (bg.r * bg.a * (1. - fg.a))) / res_alpha,
            ((fg.g * fg.a) + (bg.g * bg.a * (1. - fg.a))) / res_alpha,
            ((fg.b * fg.a) + (bg.b * bg.a * (1. - fg.a))) / res_alpha,
            res_alpha,
        )
        .into()
    }

    /// Get the resulting color from this one but with the alpha multiplied by
    /// another alpha. For example, if this color has full opacity (alpha = 255)
    /// but it's in a layer with 50% opacity (alpha = 127), the resulting color
    /// will have the same RGB but the alpha will be 127.
    pub fn with_multiplied_alpha(&self, alpha: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (((self.a as f32 / 255.) * (alpha as f32 / 255.)) * 255.) as u8,
        }
    }

    /// Get the hexadecimal representation of this color (with uppercase
    /// letters and a leading `#` sign).
    pub fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn assert_colorf32_eq(a: ColorF32, b: ColorF32) {
        dbg!(a, b);
        assert!((a.r - b.r).abs() < 0.01);
        assert!((a.g - b.g).abs() < 0.01);
        assert!((a.b - b.b).abs() < 0.01);
        assert!((a.a - b.a).abs() < 0.01);
    }

    #[test_case((255, 255, 255, 255), (1., 1., 1., 1.))]
    #[test_case((0, 0, 0, 0), (0., 0., 0., 0.))]
    #[test_case((255, 0, 0, 0), (1., 0., 0., 0.))]
    #[test_case((255, 127, 0, 0), (1., 0.5, 0., 0.))]
    #[test_case((76, 0, 189, 199), (0.3, 0., 0.745, 0.784))]
    fn convert<C: Into<Color>, CF: Into<ColorF32>>(a: C, b: CF) {
        let a: Color = a.into();
        let b: ColorF32 = b.into();
        assert_eq!(a, b.into());
        assert_colorf32_eq(b, a.into());
    }

    #[test_case((0, 0, 0, 255), (255, 255, 255, 255), (0, 0, 0, 255))]
    #[test_case((10, 200, 99, 255), (0, 255, 0, 99), (10, 200, 99, 255))]
    #[test_case((0, 0, 0, 127), (255, 255, 255, 255), (127, 127, 127, 255))]
    #[test_case((0, 0, 0, 127), (255, 255, 255, 127), (85, 85, 85, 190))]
    #[test_case((255, 0, 0, 11), (0, 255, 0, 11), (130, 124, 0, 21))]
    #[test_case((0, 0, 0, 0), (255, 255, 255, 11), (255, 255, 255, 11))]
    #[test_case((0, 0, 0, 0), (67, 127, 28, 110), (67, 127, 28, 110))]
    #[test_case((10, 100, 0, 0), (67, 127, 28, 110), (67, 127, 28, 110))]
    fn blend_color<C: Into<Color>>(a: C, b: C, res: C) {
        let a = a.into();
        let b = b.into();
        assert_eq!(a.blend_over(b), res.into(), "colors: {a:?} over {b:?}");
    }

    #[test_case((0, 0, 0, 255), 255, (0, 0, 0, 255))]
    #[test_case((0, 0, 0, 255), 127, (0, 0, 0, 127))]
    #[test_case((0, 0, 0, 127), 127, (0, 0, 0, 63))]
    #[test_case((10, 7, 255, 127), 127, (10, 7, 255, 63))]
    #[test_case((42, 69, 129, 255), 255, (42, 69, 129, 255))]
    fn multiplied_alpha<C: Into<Color>>(a: C, alpha: u8, res: C) {
        assert_eq!(a.into().with_multiplied_alpha(alpha), res.into());
    }

    #[test_case((0, 0, 0, 255), "#000000FF")]
    #[test_case((255, 255, 255, 255), "#FFFFFFFF")]
    #[test_case((127, 127, 127, 255), "#7F7F7FFF")]
    #[test_case((0, 127, 255, 0), "#007FFF00")]
    #[test_case((10, 170, 220, 199), "#0AAADCC7")]
    fn hex_val(color: impl Into<Color>, hex: &str) {
        assert_eq!(color.into().hex(), hex);
    }
}
