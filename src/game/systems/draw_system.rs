use crate::*;
use crate::assets::{ARROW_SPRITE, DANGO_EYE_SPRITE, DANGO_SPRITE, DIAG_ARROW_SPRITE, GRASS_SPRITE};
use crate::ecs::Entity;
use crate::game::components::{BulletMoveComponent, CameraComponent, GameManagerComponent, PositionComponent, SpriteComponent};
use crate::game::systems::System;
use crate::game::world::WORLD_BOUNDARIES;

const SCREEN_CENTER: (f32, f32) = (76.0, 76.0);
// Randomly generated from:
// for i in range(10):
//   print(f'Vec2 {{ x: {random.randint(-160, 160):.1f}, y: {random.randint(-160, 160):.1f} }},')
const GRASS: [Vec2; 10] = [
    Vec2 { x: 106.0, y: -70.0 },
    Vec2 { x: 2.0, y: -45.0 },
    Vec2 { x: -119.0, y: -127.0 },
    Vec2 { x: 86.0, y: 142.0 },
    Vec2 { x: -61.0, y: 36.0 },
    Vec2 { x: -1.0, y: -86.0 },
    Vec2 { x: -137.0, y: -95.0 },
    Vec2 { x: -134.0, y: -41.0 },
    Vec2 { x: 152.0, y: -54.0 },
    Vec2 { x: 13.0, y: -5.0 },
];

pub struct DrawSystem;

impl System for DrawSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let mut z_buffer = create_z_buffer(registry);
        bubble_sort(&mut z_buffer);
        for (_, (_, cam_pos)) in entities_with_components!(registry, CameraComponent, PositionComponent) {
            draw_grass(cam_pos);

            for (e, sprite_component, pos) in z_buffer.iter() {
                if sprite_component.is_visible {
                    if is_sprite_in_bound(cam_pos.pos, pos.pos, sprite_component.sprite.width as f32, sprite_component.sprite.height as f32) {
                        let draw_pos = camera_conversion(pos.pos, cam_pos.pos);
                        let sprite = sprite_component.sprite;
                        unsafe { *DRAW_COLORS = sprite.draw; }
                        blit(sprite.data, draw_pos.x as i32, draw_pos.y as i32, sprite.width, sprite.height, sprite.flags);
                    } else if !registry.has_component::<BulletMoveComponent>(*e) {
                        let draw_pos = camera_conversion(pos.pos, cam_pos.pos);
                        draw_arrow(draw_pos);
                    }
                }
            }
        }
        for (_, (gamestate, )) in entities_with_components!(registry, GameManagerComponent) {
            draw_health(gamestate.player_hp);
        }
    }
}

fn create_z_buffer(registry: &Registry) -> Vec<(Entity, &SpriteComponent, &PositionComponent)> {
    let mut sprites: Vec<(Entity, &SpriteComponent, &PositionComponent)> = vec![];
    for (&e, (sprite_component, pos, )) in entities_with_components!(registry, SpriteComponent, PositionComponent) {
        sprites.push((e, sprite_component, pos));
    }
    sprites
}

fn bubble_sort(vec: &mut Vec<(Entity, &SpriteComponent, &PositionComponent)>) {
    for i in 0..vec.len() {
        let mut has_swap = false;
        for j in 1..vec.len() - i {
            if vec.get(j - 1).abort().1.zindex > vec.get(j).abort().1.zindex {
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

fn draw_grass(cam_pos: &PositionComponent) {
    let cam_pos = cam_pos.pos;
    for grass in GRASS {
        let distance = cam_pos - grass;
        let x_parallel_world = (distance.x / WORLD_BOUNDARIES) as i32;
        let y_parallel_world = (distance.y / WORLD_BOUNDARIES) as i32;
        for x in x_parallel_world - 1..x_parallel_world + 2 {
            for y in y_parallel_world - 1..y_parallel_world + 2 {
                let new_distance = grass + Vec2 { x: x as f32 * WORLD_BOUNDARIES, y: y as f32 * WORLD_BOUNDARIES };
                if is_sprite_in_bound(cam_pos, new_distance, GRASS_SPRITE.width as f32, GRASS_SPRITE.height as f32) {
                    let camera_grass = camera_conversion(new_distance, cam_pos);
                    unsafe { *DRAW_COLORS = GRASS_SPRITE.draw; }
                    blit(GRASS_SPRITE.data, camera_grass.x as i32, camera_grass.y as i32, GRASS_SPRITE.width, GRASS_SPRITE.height, GRASS_SPRITE.flags);
                }
            }
        }
    }
}

fn draw_arrow(pos: Vec2) {
    unsafe { *DRAW_COLORS = ARROW_SPRITE.draw; }

    if pos.x >= 0.0 && pos.x <= 160.0 {
        if pos.y < SCREEN_CENTER.1 {
            blit(ARROW_SPRITE.data, pos.x as i32, 8 as i32, ARROW_SPRITE.width, ARROW_SPRITE.height, ARROW_SPRITE.flags | BLIT_FLIP_X | BLIT_ROTATE);
        } else {
            blit(ARROW_SPRITE.data, pos.x as i32, 144, ARROW_SPRITE.width, ARROW_SPRITE.height, ARROW_SPRITE.flags | BLIT_ROTATE);
        }
    } else if pos.y >= 0.0 && pos.y <= 160.0 {
        if pos.x < SCREEN_CENTER.0 {
            blit(ARROW_SPRITE.data, 0, pos.y as i32, ARROW_SPRITE.width, ARROW_SPRITE.height, ARROW_SPRITE.flags);
        } else {
            blit(ARROW_SPRITE.data, 152, pos.y as i32, ARROW_SPRITE.width, ARROW_SPRITE.height, ARROW_SPRITE.flags | BLIT_FLIP_X);
        }
    } else if pos.x < SCREEN_CENTER.0 {
        if pos.y < SCREEN_CENTER.1 {
            blit(DIAG_ARROW_SPRITE.data, 0, 8, DIAG_ARROW_SPRITE.width, DIAG_ARROW_SPRITE.height, DIAG_ARROW_SPRITE.flags | BLIT_FLIP_X);
        } else {
            blit(DIAG_ARROW_SPRITE.data, 0, 144, DIAG_ARROW_SPRITE.width, DIAG_ARROW_SPRITE.height, DIAG_ARROW_SPRITE.flags | BLIT_FLIP_X | BLIT_FLIP_Y);
        }
    } else {
        if pos.y < SCREEN_CENTER.1 {
            blit(DIAG_ARROW_SPRITE.data, 152, 8, DIAG_ARROW_SPRITE.width, DIAG_ARROW_SPRITE.height, DIAG_ARROW_SPRITE.flags);
        } else {
            blit(DIAG_ARROW_SPRITE.data, 152, 144, DIAG_ARROW_SPRITE.width, DIAG_ARROW_SPRITE.height, DIAG_ARROW_SPRITE.flags | BLIT_FLIP_Y);
        }
    }
}

fn is_sprite_in_bound(cam_pos: Vec2, pos: Vec2, width: f32, height: f32) -> bool {
    is_point_in_bound(cam_pos, pos) || is_point_in_bound(cam_pos, pos + Vec2 { x: width, y: height })
}

fn is_point_in_bound(cam_pos: Vec2, pos: Vec2) -> bool {
    let new_pos = camera_conversion(pos, cam_pos);

    if new_pos.x >= 0.0 && new_pos.x <= 160.0 && new_pos.y >= 0.0 && new_pos.y <= 160.0 {
        return true;
    }

    return false;
}

fn camera_conversion(pos: Vec2, cam_pos: Vec2) -> Vec2 {
    let center = Vec2::new(SCREEN_CENTER.0, SCREEN_CENTER.1);
    pos - cam_pos + center
}
