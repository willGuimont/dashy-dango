use std::any::Any;

use ecs_macro::Component;

use crate::ecs::BaseComponent;

#[derive(Component, Clone, Debug)]
pub struct GamepadComponent {
    pub gamepad: *const u8,
}

impl GamepadComponent {
    pub fn get_gamepad(&self) -> u8 {
        unsafe { *self.gamepad }
    }
}
