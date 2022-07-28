use std::any::Any;

use ecs_macro::Component;

use crate::BaseComponent;

#[derive(Component, Clone, Debug)]
pub struct DashComponent {
    pub dash: i16,
    pub timeout: i16,
}
 