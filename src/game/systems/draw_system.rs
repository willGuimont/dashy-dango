use crate::*;
use crate::ecs::Entity;
use crate::game::components::{CameraComponent, PositionComponent, SpriteComponent};
use crate::game::systems::System;

const SCREEN_CENTER: (f32, f32) = (76.0, 76.0);

pub struct DrawSystem;

impl System for DrawSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let mut z_buffer = create_z_buffer(registry);
        bubble_sort(&mut z_buffer);
        for (_, (_, cam_pos)) in entities_with_components!(registry, CameraComponent, PositionComponent) {
            for (sprite_component, pos) in z_buffer.iter() {
                let new_pos = camera_conversion(pos, cam_pos);
                let sprite = sprite_component.sprite;
                unsafe { *DRAW_COLORS = sprite.draw; }
                blit(sprite.data, new_pos.x as i32, new_pos.y as i32, sprite.width, sprite.height, sprite.flags);
            }
        }
    }
}

fn create_z_buffer(registry: &Registry) -> Vec<(&SpriteComponent, &PositionComponent)> {
    let mut sprites: Vec<(&SpriteComponent, &PositionComponent)> = vec![];
    for (_, (sprite_component, pos, )) in entities_with_components!(registry, SpriteComponent, PositionComponent) {
        sprites.push((sprite_component, pos));
    }
    sprites
}

fn bubble_sort(vec: &mut Vec<(&SpriteComponent, &PositionComponent)>) {
    for i in 0..vec.len() {
        let mut has_swap = false;
        for j in 1..vec.len() - i {
            if vec.get(j - 1).abort().0.zindex > vec.get(j).abort().0.zindex {
                has_swap = true;
                vec.swap(j - 1, j);
            }
        }
        if !has_swap {
            return;
        }
    }
}

fn camera_conversion(pos: &PositionComponent, cam_pos: &PositionComponent) -> Vec2 {
    let center = Vec2::new(SCREEN_CENTER.0, SCREEN_CENTER.1);
    pos.pos - cam_pos.pos + center
}
