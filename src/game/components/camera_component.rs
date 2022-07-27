use std::any::Any;

use ecs_macro::Component;

use crate::ecs::BaseComponent;

#[derive(Component, Debug, Clone)]
pub struct CameraComponent {}