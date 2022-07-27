use std::any::Any;
use ecs_macro::Component;
use crate::ecs::BaseComponent;

#[derive(Component, Debug)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}