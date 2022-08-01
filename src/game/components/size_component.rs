use std::any::Any;

use ecs_macro::Component;

use crate::BaseComponent;

#[derive(Component, Clone, Debug)]
pub struct SizeComponent {
    pub width: i16,
    pub height: i16,
}
