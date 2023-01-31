use crate::ui_state::{UiEvent, UiState};
use crate::wrapped_image::WrappedImage;
use crate::{Bitmap, Event};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Shortcut {
    KeyPress(KeyCode),
    KeyPressMod(Modifier, KeyCode),
    KeyDown(KeyCode),
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Modifier {
    Ctrl,
    Shift,
}

#[derive(Debug, Clone)]
pub enum Effect {
    Event(Event<WrappedImage>),
    UiEvent(UiEvent),
}

pub struct Shortcuts(HashMap<Shortcut, Effect>);

impl Shortcuts {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn register(&mut self, shortcut: Shortcut, effect: Effect) {
        self.0.insert(shortcut, effect);
    }

    pub fn register_keypress_event(&mut self, key: KeyCode, event: Event<WrappedImage>) {
        self.0.insert(Shortcut::KeyPress(key), Effect::Event(event));
    }

    pub fn register_keypress_mod_event(
        &mut self,
        modifier: Modifier,
        key: KeyCode,
        event: Event<WrappedImage>,
    ) {
        self.0
            .insert(Shortcut::KeyPressMod(modifier, key), Effect::Event(event));
    }

    pub fn register_keypress_ui_event(&mut self, key: KeyCode, event: UiEvent) {
        self.0
            .insert(Shortcut::KeyPress(key), Effect::UiEvent(event));
    }

    pub fn register_keydown_ui_event(&mut self, key: KeyCode, event: UiEvent) {
        self.0
            .insert(Shortcut::KeyDown(key), Effect::UiEvent(event));
    }

    pub fn process(&self) -> Vec<Effect> {
        let mut fx = Vec::new();

        for (shortcut, effect) in &self.0 {
            let execute = match shortcut {
                Shortcut::KeyPress(key) => is_key_pressed(*key),
                Shortcut::KeyDown(key) => is_key_down(*key),
                Shortcut::KeyPressMod(modif, key) => match modif {
                    Modifier::Ctrl => {
                        (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
                            && is_key_pressed(*key)
                    }
                    Modifier::Shift => {
                        (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                            && is_key_pressed(*key)
                    }
                },
                _ => false,
            };

            if execute {
                fx.push(effect.clone());
            }
        }

        fx
    }
}
