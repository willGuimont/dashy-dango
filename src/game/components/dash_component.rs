use std::any::Any;

use ecs_macro::Component;

use crate::{BaseComponent, Vec2};

#[derive(Component, Clone, Debug)]
pub struct DashComponent {
    pub dash: i16,
    pub timeout: i16,
    pub is_dashing: i16,
    pub direction: Vec2,
}
 