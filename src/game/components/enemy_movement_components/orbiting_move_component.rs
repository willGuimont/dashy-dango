use std::any::Any;

use ecs_macro::Component;

use crate::ecs::{BaseComponent, Entity};
use crate::Vec2;

#[derive(Component, Clone, Debug)]
pub struct OrbitingMoveComponent {
    pub last_pos: Vec2,
    pub target: Entity,
    pub perp_speed: f32,
}
