use std::any::Any;
use std::collections::HashSet;

use ecs_macro::Component;

use crate::{BaseComponent, Vec2};
use crate::ecs::Entity;

#[derive(Component, Clone, Debug)]
pub struct DashComponent {
    pub length: i16,
    pub timeout: i16,
    pub duration: i16,
    pub direction: Vec2,
    pub hit: HashSet<Entity>,
    pub grace_period: i16,
}
 