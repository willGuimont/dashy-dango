use std::any::Any;

use ecs_macro::Component;

use crate::ecs::{BaseComponent, Entity};

#[derive(Component, Clone, Debug)]
pub struct ShooterComponent {
    pub bullet_speed: f32,
    pub firing_timeout: i16,
    pub firing_delay: i16,
    pub bullet_lifespan: i16,
    pub firing_distance: i16,
    pub target: Entity,
}
