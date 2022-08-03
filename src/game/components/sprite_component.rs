use std::any::Any;

use ecs_macro::Component;

use crate::assets::Sprite;
use crate::BaseComponent;

#[derive(Component, Clone, Debug)]
pub struct SpriteComponent {
    pub width: u32,
    pub heigt: u32,
    pub flags: u32,
    pub data: &'static [u8],
}

impl SpriteComponent {
    pub fn new(sprite: Sprite) -> Self {
        SpriteComponent { width: sprite.width, heigt: sprite.height, flags: sprite.flags, data: sprite.data }
    }
}
