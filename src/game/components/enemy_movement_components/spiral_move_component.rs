use std::any::Any;

use ecs_macro::Component;

use crate::ecs::BaseComponent;

#[derive(Component, Clone, Debug)]
pub struct SpiralMoveComponent {
    pub perpendicular_speed: f32,
    pub parallel_speed: f32,
}
