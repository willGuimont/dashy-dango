use std::any::Any;

use ecs_macro::Component;

use crate::ecs::BaseComponent;

#[derive(Component, Clone, Debug)]
pub struct GameManagerComponent {
    pub current_wave: u8,
    pub score: i32,
    pub player_hp: i16,
}
