pub trait Color: Copy {
    fn rgb(&self) -> (u8, u8, u8);
    fn rgba(&self) -> (u8, u8, u8, u8);
    fn rgb_f32(&self) -> (f32, f32, f32);
    fn rgba_f32(&self) -> (f32, f32, f32, f32);
    fn from_rgb(r: u8, g: u8, b: u8) -> Self;
    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self;
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
