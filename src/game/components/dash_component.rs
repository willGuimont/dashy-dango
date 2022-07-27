use std::any::Any;
use crate::BaseComponent;
use ecs_macro::Component;

#[derive(Component, Debug)]
pub struct DashComponent {
    pub dash:i16,
    pub timeout:i16
}