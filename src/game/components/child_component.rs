use std::any::Any;

use ecs_macro::Component;

use crate::ecs::{BaseComponent, Entity};
use crate::Vec2;

#[derive(Component, Clone, Debug)]
pub struct ChildComponent {
    pub parent: Entity,
    pub relative_pos: Vec2,
}
