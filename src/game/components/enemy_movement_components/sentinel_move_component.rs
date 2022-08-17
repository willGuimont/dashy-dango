use std::any::Any;

use ecs_macro::Component;

use crate::ecs::{BaseComponent, Entity};

#[derive(Component, Clone, Debug)]
pub struct SentinelMoveComponent {
    pub speed: f32,
    pub stopping_distance: f32,
    pub target: Entity,
}
