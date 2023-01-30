use std::default::Default;
use lapix_core::{Canvas, Event, State, Tool};
use crate::wrapped_image::WrappedImage;

pub const WINDOW_W: i32 = 1000;
pub const WINDOW_H: i32 = 600;
pub const CANVAS_W: u16 = 64;
pub const CANVAS_H: u16 = 64;
const CANVAS_SCALE: f32 = 8.;
const LEFT_TOOLBAR_W: u16 = 300;

// Center on the space after the toolbar
const CANVAS_X: f32 = LEFT_TOOLBAR_W as f32 + ((WINDOW_W as u16 - LEFT_TOOLBAR_W) / 2) as f32
    - (CANVAS_W as f32 * CANVAS_SCALE / 2.);
const CANVAS_Y: f32 = (WINDOW_H / 2) as f32 - (CANVAS_H as f32 * CANVAS_SCALE / 2.);

pub trait Number {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for f32 {}
impl Number for f64 {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point<T: Number> {
    pub x: T,
    pub y: T
}
pub type Position<T> = Point<T>;
pub type Size<T> = Point<T>;

impl<T: Number> From<(T, T)> for Position<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct UiState {
    inner: State<WrappedImage>,
    camera: Position<f32>,
    canvas_pos: Position<f32>,
    zoom: f32
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            inner: State::<WrappedImage>::new(CANVAS_W, CANVAS_H),
            camera: (0., 0.).into(),
            canvas_pos: (CANVAS_X, CANVAS_Y).into(),
            zoom: 8.
        }
    }
}

impl UiState {
    pub fn camera(&self) -> Position<f32> {
        self.camera
    }
    pub fn canvas(&self) -> &Canvas<WrappedImage> {
        &self.inner.canvas()
    }
    pub fn execute(&mut self, event: Event<WrappedImage>) {
        self.inner.execute(event);
    }
    pub fn canvas_pos(&self) -> Position<f32> {
        self.canvas_pos
    }
    pub fn canvas_size(&self) -> Size<f32> {
        (self.inner.canvas().width() as f32,
        self.inner.canvas().height() as f32).into()
    }
    pub fn canvas_actual_size(&self) -> Size<f32> {
        (self.inner.canvas().width() as f32 * self.zoom,
        self.inner.canvas().height() as f32 * self.zoom).into()
    }
    pub fn main_color(&self) -> [u8; 4] {
        self.inner.main_color()
    }
    pub fn selected_tool(&self) -> Tool {
        self.inner.selected_tool()
    }
    pub fn zoom(&self) -> f32 {
        self.zoom
    }
    pub fn zoom_in(&mut self) {
        self.zoom *= 2.;
    }
    pub fn zoom_out(&mut self) {
        self.zoom /= 2.;
    }
    pub fn move_camera(&mut self, direction: Direction) {
        let speed = 0.5 * self.zoom;

        if !self.is_camera_off(direction) {
            match direction {
                Direction::Up => self.camera.y -= speed,
                Direction::Down => self.camera.y += speed,
                Direction::Left => self.camera.x -= speed,
                Direction::Right => self.camera.x += speed,
            }
        }
    }
    fn is_camera_off(&self, direction: Direction) -> bool {
        let buffer = 20.;
        let canvas_size = self.canvas_actual_size();
        let canvas_pos = self.canvas_pos;
        let camera = self.camera;
        let win_w = WINDOW_W as f32;
        let win_h = WINDOW_H as f32;

        match direction {
            Direction::Up => canvas_pos.y - camera.y > win_h as f32 - buffer,
            Direction::Down => camera.y > canvas_pos.y + canvas_size.y - buffer,
            Direction::Left => canvas_pos.x - camera.x > win_w as f32 - buffer,
            Direction::Right => camera.x > canvas_pos.x + canvas_size.x - buffer,
        }
    }
}

