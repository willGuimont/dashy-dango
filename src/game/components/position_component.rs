use std::any::Any;

use ecs_macro::Component;

use crate::ecs::BaseComponent;
use crate::Vec2;

#[derive(Component, Clone, Debug)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}

impl PositionComponent {
    pub fn to_vec(&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }

    pub fn from_vec(vec: Vec2) -> PositionComponent {
        PositionComponent { x: vec.x, y: vec.y }
    }
}
