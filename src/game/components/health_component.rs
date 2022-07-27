use std::any::Any;
use ecs_macro::Component;
use crate::ecs::BaseComponent;

#[derive(Component, Debug)]
pub struct HealthComponent {
    pub hp: i16,
}