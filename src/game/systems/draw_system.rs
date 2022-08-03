use crate::*;
use crate::assets::sprites::*;
use crate::ecs::Entity;
use crate::game::components::{CameraComponent, PositionComponent, SpriteComponent};
use crate::game::systems::System;

const SCREEN_CENTER: (f32, f32) = (76.0, 76.0);

//TODO add spriteComponent
#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];


pub struct DrawSystem;

impl System for DrawSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for (_, (_, cam_pos)) in entities_with_components!(registry, CameraComponent, PositionComponent) {
            for (_, (sprite_component, pos, )) in entities_with_components!(registry, SpriteComponent, PositionComponent) {
                let new_pos = camera_conversion(pos, cam_pos);
                let sprite = &sprite_component.sprite;
                blit(sprite.data, new_pos.x as i32, new_pos.y as i32, sprite.width, sprite.height, sprite.flags);
            }
        }
    }
}

fn camera_conversion(pos: &PositionComponent, cam_pos: &PositionComponent) -> Vec2 {
    let center = Vec2::new(SCREEN_CENTER.0, SCREEN_CENTER.1);
    pos.pos - cam_pos.pos + center
}
