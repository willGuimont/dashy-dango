use std::any::Any;

use ecs_macro::Component;

use crate::assets::Sprite;
use crate::BaseComponent;

#[derive(Component, Clone)]
pub struct SpriteComponent {
    pub sprite: &'static Sprite,
    pub zindex: u8,
    pub is_visible: bool,
}

