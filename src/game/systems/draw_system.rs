use crate::*;
use crate::ecs::Entity;
use crate::game::components::{CameraComponent, PositionComponent, SpriteComponent};
use crate::game::systems::System;

const SCREEN_CENTER: (f32, f32) = (76.0, 76.0);

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
