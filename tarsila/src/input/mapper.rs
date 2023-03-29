use super::{InputEvent, KeyBindings};
use crate::Effect;

#[derive(Debug)]
pub struct InputMapper;

impl InputMapper {
    pub fn map(&self, key_bindings: &KeyBindings, input_events: Vec<InputEvent>) -> Vec<Effect> {
        if !input_events.is_empty() {
            dbg!(&input_events);
        }

        let mut fx = Vec::new();

        for (keys, action) in key_bindings.iter() {
            if keys.matches(&input_events) {
                let mut new_fx = action.produce(&input_events);
                fx.append(&mut new_fx);
            }
        }

        if !fx.is_empty() {
            dbg!(&fx);
        }
        fx
    }
}
