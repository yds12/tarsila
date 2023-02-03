use crate::ui_state::UiEvent;
use crate::wrapped_image::WrappedImage;
use lapix_core::{Direction, Event, Tool};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::time::SystemTime;

const KEYDOWN_INTERVAL: u128 = 5;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Shortcut {
    KeyPress(KeyCode),
    KeyPressMod(Modifier, KeyCode),
    KeyDown(KeyCode),
    KeyDownMod(Modifier, KeyCode),
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

pub struct KeyboardManager {
    shortcuts: HashMap<Shortcut, Effect>,
    last_keydown: Option<SystemTime>,
}

impl KeyboardManager {
    pub fn new() -> Self {
        let mut km = Self {
            shortcuts: HashMap::new(),
            last_keydown: None,
        };
        km.register_defaults();

        km
    }

    pub fn register_defaults(&mut self) {
        let kv = [
            (KeyCode::Equal, UiEvent::ZoomIn),
            (KeyCode::Minus, UiEvent::ZoomOut),
        ];
        for (k, v) in kv {
            self.register_keypress_ui_event(k, v);
        }

        let kv = [
            (KeyCode::Up, UiEvent::MoveCamera(Direction::Up)),
            (KeyCode::Down, UiEvent::MoveCamera(Direction::Down)),
            (KeyCode::Left, UiEvent::MoveCamera(Direction::Left)),
            (KeyCode::Right, UiEvent::MoveCamera(Direction::Right)),
        ];
        for (k, v) in kv {
            self.register_keydown_ui_event(k, v);
        }

        let kv = [
            (KeyCode::B, Event::SetTool(Tool::Brush)),
            (KeyCode::E, Event::SetTool(Tool::Eraser)),
            (KeyCode::G, Event::SetTool(Tool::Bucket)),
            (KeyCode::I, Event::SetTool(Tool::Eyedropper)),
            (KeyCode::L, Event::SetTool(Tool::Line)),
            (KeyCode::Y, Event::NewLayerAbove),
        ];
        for (k, v) in kv {
            self.register_keypress_event(k, v);
        }

        self.register_keypress_mod_event(Modifier::Ctrl, KeyCode::Z, Event::Undo);
        //self.register_keydown_mod_event(Modifier::Ctrl, KeyCode::Z, Event::Undo);
    }

    pub fn register_keydown(&mut self) {
        self.last_keydown = Some(SystemTime::now());
    }

    pub fn allow_keydown(&self) -> bool {
        match self.last_keydown {
            Some(time) => time.elapsed().unwrap().as_millis() > KEYDOWN_INTERVAL,
            None => true,
        }
    }

    pub fn register(&mut self, shortcut: Shortcut, effect: Effect) {
        self.shortcuts.insert(shortcut, effect);
    }

    pub fn register_keypress_event(&mut self, key: KeyCode, event: Event<WrappedImage>) {
        self.register(Shortcut::KeyPress(key), Effect::Event(event));
    }

    pub fn register_keydown_event(&mut self, key: KeyCode, event: Event<WrappedImage>) {
        self.register(Shortcut::KeyDown(key), Effect::Event(event));
    }

    pub fn register_keypress_mod_event(
        &mut self,
        modifier: Modifier,
        key: KeyCode,
        event: Event<WrappedImage>,
    ) {
        self.register(Shortcut::KeyPressMod(modifier, key), Effect::Event(event));
    }

    pub fn register_keydown_mod_event(
        &mut self,
        modifier: Modifier,
        key: KeyCode,
        event: Event<WrappedImage>,
    ) {
        self.register(Shortcut::KeyDownMod(modifier, key), Effect::Event(event));
    }

    pub fn register_keypress_ui_event(&mut self, key: KeyCode, event: UiEvent) {
        self.register(Shortcut::KeyPress(key), Effect::UiEvent(event));
    }

    pub fn register_keydown_ui_event(&mut self, key: KeyCode, event: UiEvent) {
        self.register(Shortcut::KeyDown(key), Effect::UiEvent(event));
    }

    pub fn update(&mut self) -> Vec<Effect> {
        let mut fx = Vec::new();

        for (shortcut, effect) in &self.shortcuts {
            let execute = match shortcut {
                Shortcut::KeyPress(key) => is_key_pressed(*key),
                Shortcut::KeyDown(key) => is_key_down(*key) && self.allow_keydown(),
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
                Shortcut::KeyDownMod(modif, key) => match modif {
                    Modifier::Ctrl => {
                        (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
                            && is_key_down(*key)
                            && self.allow_keydown()
                    }
                    Modifier::Shift => {
                        (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                            && is_key_down(*key)
                            && self.allow_keydown()
                    }
                },
            };

            if execute {
                fx.push(effect.clone());
                //break;
            }
        }

        if !fx.is_empty() {
            self.register_keydown();
        }

        fx
    }
}
