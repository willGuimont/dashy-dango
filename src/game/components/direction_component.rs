use std::any::Any;
use ecs_macro::Component;
use crate::ecs::BaseComponent;
use crate::Vec2;

#[derive(Component, Debug, Clone)]
pub struct DirectionComponent {
    pub direction: Vec2,
}