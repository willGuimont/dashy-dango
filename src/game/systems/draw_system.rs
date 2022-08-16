use crate::*;
use crate::assets::{DANGO_EYE_SPRITE, DANGO_SPRITE};
use crate::ecs::Entity;
use crate::game::components::{CameraComponent, GameManagerComponent, PositionComponent, SpriteComponent};
use crate::game::systems::System;

const SCREEN_CENTER: (f32, f32) = (76.0, 76.0);

pub struct DrawSystem;

impl System for DrawSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let mut z_buffer = create_z_buffer(registry);
        bubble_sort(&mut z_buffer);
        for (_, (_, cam_pos)) in entities_with_components!(registry, CameraComponent, PositionComponent) {
            for (sprite_component, pos) in z_buffer.iter() {
                if sprite_component.is_visible {
                    let new_pos = camera_conversion(pos, cam_pos);
                    let sprite = sprite_component.sprite;
                    unsafe { *DRAW_COLORS = sprite.draw; }
                    blit(sprite.data, new_pos.x as i32, new_pos.y as i32, sprite.width, sprite.height, sprite.flags);
                }
            }
        }
        for (e, (gamestate, )) in entities_with_components!(registry, GameManagerComponent) {
            draw_health(gamestate.player_hp);
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

fn draw_health(player_health: i16) {
    unsafe { *DRAW_COLORS = 0x0002; }
    rect(0, 0, 160, 8);
    for i in 0..player_health {
        unsafe { *DRAW_COLORS = 0x0342; }
        blit(DANGO_SPRITE.data, (160 - (i + 1) * 8) as i32, 0, DANGO_SPRITE.width, DANGO_SPRITE.height, DANGO_SPRITE.flags);
        unsafe { *DRAW_COLORS = DANGO_EYE_SPRITE.draw; }
        blit(DANGO_EYE_SPRITE.data, 3 + (160 - (i + 1) * 8) as i32, 4, DANGO_EYE_SPRITE.width, DANGO_EYE_SPRITE.height, DANGO_EYE_SPRITE.flags);
    }
}

fn camera_conversion(pos: &PositionComponent, cam_pos: &PositionComponent) -> Vec2 {
    let center = Vec2::new(SCREEN_CENTER.0, SCREEN_CENTER.1);
    pos.pos - cam_pos.pos + center
}
