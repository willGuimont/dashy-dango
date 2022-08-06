use std::any::Any;

use ecs_macro::Component;

use crate::ecs::BaseComponent;
use crate::Vec2;

#[derive(Component, Clone, Debug)]
pub struct BulletMoveComponent {
    pub speed: f32,
    pub direction: Vec2,
}
