use std::any::Any;

use ecs_macro::Component;

use crate::ecs::BaseComponent;

#[derive(Component, Clone, Debug)]
pub struct HealthComponent {
    pub hp: i16,
    pub timeout: i16,
    pub hit_delay: i16,
}
