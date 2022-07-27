use std::any::Any;

use ecs_macro::Component;

use crate::ecs::BaseComponent;

#[derive(Component, Debug, Clone)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}