use super::{InputEvent, InputMapper, KeyBindings, KeyboardKey, KeyboardModifier, MouseButton};
use crate::Effect;
use lapix::{Direction, Position};
use macroquad::prelude as mq;

#[derive(Debug)]
pub struct InputManager {
    keys_to_track: Vec<KeyboardKey>,
    mapper: InputMapper,
    prev_mouse_canvas: Position<i32>,
    mouse_canvas: Position<i32>,
    mouse: Position<f32>,
    prev_mouse: Position<f32>,
}

impl InputManager {
    pub fn new(keys_to_track: Vec<KeyboardKey>) -> Self {
        Self {
            keys_to_track,
            mapper: InputMapper,
            prev_mouse_canvas: Default::default(),
            mouse_canvas: Default::default(),
            mouse: Default::default(),
            prev_mouse: Default::default(),
        }
    }

    pub fn sync(&mut self, mouse_pos: Position<f32>, mouse_canvas_pos: Position<i32>) {
        self.prev_mouse_canvas = self.mouse_canvas;
        self.mouse_canvas = mouse_canvas_pos;
        self.prev_mouse = self.mouse;
        self.mouse = mouse_pos;
    }

    pub fn update(&self, key_bindings: &KeyBindings) -> Vec<Effect> {
        let input_events = self.get_input_events();
        let fx = self.mapper.map(key_bindings, input_events);
        fx
    }

    fn get_input_events(&self) -> Vec<InputEvent> {
        let mut events = Vec::new();

        // mouse

        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            events.push(InputEvent::MouseButtonPress(MouseButton(
                mq::MouseButton::Left,
            )));
        }
        if mq::is_mouse_button_pressed(mq::MouseButton::Right) {
            events.push(InputEvent::MouseButtonPress(MouseButton(
                mq::MouseButton::Right,
            )));
        }

        if mq::is_mouse_button_down(mq::MouseButton::Left) {
            events.push(InputEvent::MouseButtonDown(MouseButton(
                mq::MouseButton::Left,
            )));
        }
        if mq::is_mouse_button_down(mq::MouseButton::Right) {
            events.push(InputEvent::MouseButtonDown(MouseButton(
                mq::MouseButton::Right,
            )));
        }

        if mq::is_mouse_button_released(mq::MouseButton::Left) {
            events.push(InputEvent::MouseButtonRelease(MouseButton(
                mq::MouseButton::Left,
            )));
        }
        if mq::is_mouse_button_released(mq::MouseButton::Right) {
            events.push(InputEvent::MouseButtonRelease(MouseButton(
                mq::MouseButton::Right,
            )));
        }

        if self.prev_mouse_canvas != self.mouse_canvas {
            events.push(InputEvent::MouseCanvasMove(
                self.mouse_canvas - self.prev_mouse_canvas,
            ));
        }

        if self.prev_mouse != self.mouse {
            events.push(InputEvent::MouseRealMove(
                (self.mouse - self.prev_mouse).into(),
            ));
        }

        let scroll = mq::mouse_wheel().1;
        if scroll > 0. {
            events.push(InputEvent::MouseScrollUp);
        } else if scroll < 0. {
            events.push(InputEvent::MouseScrollDown);
        }

        // keyboard

        for key in &self.keys_to_track {
            if mq::is_key_pressed(key.0) {
                events.push(InputEvent::KeyPress(key.clone()));
            }
            if mq::is_key_down(key.0) {
                events.push(InputEvent::KeyDown(key.clone()));
            }
            if mq::is_key_released(key.0) {
                events.push(InputEvent::KeyRelease(key.clone()));
            }
        }

        if mq::is_key_down(mq::KeyCode::RightShift) || mq::is_key_down(mq::KeyCode::LeftShift) {
            events.push(InputEvent::KeyModifier(KeyboardModifier::Shift));
        }
        if mq::is_key_down(mq::KeyCode::RightControl) || mq::is_key_down(mq::KeyCode::LeftControl) {
            events.push(InputEvent::KeyModifier(KeyboardModifier::Control));
        }
        if mq::is_key_down(mq::KeyCode::RightAlt) || mq::is_key_down(mq::KeyCode::LeftAlt) {
            events.push(InputEvent::KeyModifier(KeyboardModifier::Alt));
        }
        if mq::is_key_down(mq::KeyCode::RightSuper) || mq::is_key_down(mq::KeyCode::LeftSuper) {
            events.push(InputEvent::KeyModifier(KeyboardModifier::Super));
        }

        events
    }
}
