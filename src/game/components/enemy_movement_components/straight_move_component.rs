use std::any::Any;

use ecs_macro::Component;

use crate::ecs::{BaseComponent, Entity};

#[derive(Component, Clone, Debug)]
pub struct StraightMoveComponent {
    pub speed: f32,
    pub target: Entity,
}
