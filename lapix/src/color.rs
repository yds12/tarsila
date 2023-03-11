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

        ColorF32::new(
            (fg.r * fg.a) + (bg.r * bg.a * (1. - fg.a)),
            (fg.g * fg.a) + (bg.g * bg.a * (1. - fg.a)),
            (fg.b * fg.a) + (bg.b * bg.a * (1. - fg.a)),
            fg.a + bg.a * (1. - fg.a),
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

    #[test_case((0, 0, 0, 255), (255, 255, 255, 255), (0, 0, 0, 255))]
    #[test_case((10, 200, 99, 255), (0, 255, 0, 99), (10, 200, 99, 255))]
    #[test_case((0, 0, 0, 127), (255, 255, 255, 255), (127, 127, 127, 255))]
    #[test_case((0, 0, 0, 127), (255, 255, 255, 127), (63, 63, 63, 190))]
    fn blend_color<C: Into<Color>>(a: C, b: C, res: C) {
        assert_eq!(a.into().blend_over(b.into()), res.into());
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
