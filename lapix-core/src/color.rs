use std::fmt::Debug;

pub trait Color: Debug + Copy + PartialEq<Self> {
    fn rgb(&self) -> (u8, u8, u8);
    fn rgba(&self) -> (u8, u8, u8, u8);
    fn rgb_f32(&self) -> (f32, f32, f32);
    fn rgba_f32(&self) -> (f32, f32, f32, f32);
    fn from_rgb(r: u8, g: u8, b: u8) -> Self;
    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self;

    /// Blend this color on top of another
    fn blend_over(&self, other: Self) -> Self {
        let fg = self.rgba_f32();
        let bg = other.rgba_f32();

        let res = (
            (fg.0 * fg.3) + (bg.0 * bg.3 * (1. - fg.3)),
            (fg.1 * fg.3) + (bg.1 * bg.3 * (1. - fg.3)),
            (fg.2 * fg.3) + (bg.2 * bg.3 * (1. - fg.3)),
            fg.3 + bg.3 * (1. - fg.3),
        );

        Self::from_rgba(
            (res.0 * 255.) as u8,
            (res.1 * 255.) as u8,
            (res.2 * 255.) as u8,
            (res.3 * 255.) as u8,
        )
    }
}

impl Color for [u8; 3] {
    fn rgb(&self) -> (u8, u8, u8) {
        (self[0], self[1], self[2])
    }
    fn rgba(&self) -> (u8, u8, u8, u8) {
        (self[0], self[1], self[2], 255)
    }
    fn rgb_f32(&self) -> (f32, f32, f32) {
        (
            self[0] as f32 / 255.,
            self[1] as f32 / 255.,
            self[2] as f32 / 255.,
        )
    }
    fn rgba_f32(&self) -> (f32, f32, f32, f32) {
        (
            self[0] as f32 / 255.,
            self[1] as f32 / 255.,
            self[2] as f32 / 255.,
            1.,
        )
    }
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        [r, g, b]
    }
    fn from_rgba(r: u8, g: u8, b: u8, _: u8) -> Self {
        [r, g, b]
    }
}

impl Color for (u8, u8, u8) {
    fn rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
    fn rgba(&self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, 255)
    }
    fn rgb_f32(&self) -> (f32, f32, f32) {
        (
            self.0 as f32 / 255.,
            self.1 as f32 / 255.,
            self.2 as f32 / 255.,
        )
    }
    fn rgba_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.0 as f32 / 255.,
            self.1 as f32 / 255.,
            self.2 as f32 / 255.,
            1.,
        )
    }
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        (r, g, b)
    }
    fn from_rgba(r: u8, g: u8, b: u8, _: u8) -> Self {
        (r, g, b)
    }
}

impl Color for [u8; 4] {
    fn rgb(&self) -> (u8, u8, u8) {
        (self[0], self[1], self[2])
    }
    fn rgba(&self) -> (u8, u8, u8, u8) {
        (self[0], self[1], self[2], self[3])
    }
    fn rgb_f32(&self) -> (f32, f32, f32) {
        (
            self[0] as f32 / 255.,
            self[1] as f32 / 255.,
            self[2] as f32 / 255.,
        )
    }
    fn rgba_f32(&self) -> (f32, f32, f32, f32) {
        (
            self[0] as f32 / 255.,
            self[1] as f32 / 255.,
            self[2] as f32 / 255.,
            self[3] as f32 / 255.,
        )
    }
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        [r, g, b, 255]
    }
    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        [r, g, b, a]
    }
}

impl Color for (u8, u8, u8, u8) {
    fn rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
    fn rgba(&self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }
    fn rgb_f32(&self) -> (f32, f32, f32) {
        (
            self.0 as f32 / 255.,
            self.1 as f32 / 255.,
            self.2 as f32 / 255.,
        )
    }
    fn rgba_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.0 as f32 / 255.,
            self.1 as f32 / 255.,
            self.2 as f32 / 255.,
            self.3 as f32 / 255.,
        )
    }
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        (r, g, b, 255)
    }
    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        (r, g, b, a)
    }
}
