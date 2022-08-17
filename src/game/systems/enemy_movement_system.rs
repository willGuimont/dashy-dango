use crate::{Abort, create_box, entities_with, entities_with_components, get_components_clone_unwrap, get_components_unwrap, has_all_components, Registry, Topic, trace, Vec2};
use crate::ecs::Entity;
use crate::game::components::{DashComponent, EnemyComponent, GameManagerComponent, HealthComponent, PlayerComponent, PositionComponent, SizeComponent, TombstoneComponent};
use crate::game::components::bullet_move_component::BulletMoveComponent;
use crate::game::components::sentinel_move_component::SentinelMoveComponent;
use crate::game::components::spiral_move_component::SpiralMoveComponent;
use crate::game::components::straight_move_component::StraightMoveComponent;
use crate::game::systems::System;

pub struct EnemyMovementSystem {
    pub damage_topic: Topic<(Entity, i32, i32)>,
}

impl System for EnemyMovementSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry, EnemyComponent) {
            let (enemy_pos, enemy_size) = get_components_clone_unwrap!(registry, e, PositionComponent, SizeComponent);
            let mut enemy_pos: Vec2 = enemy_pos.pos;
            if registry.has_component::<StraightMoveComponent>(e) {
                let (movement, ) = get_components_unwrap!(registry, e, StraightMoveComponent);
                let pos_and_is_alive = get_target_lifelihood_and_position(registry, movement.target);
                enemy_pos = straight_move(enemy_pos, pos_and_is_alive.0, movement, pos_and_is_alive.1);
            } else if registry.has_component::<SpiralMoveComponent>(e) {
                let (movement, ) = get_components_unwrap!(registry, e, SpiralMoveComponent);
                let pos_and_is_alive = get_target_lifelihood_and_position(registry, movement.target);
                enemy_pos = spiral_move(enemy_pos, pos_and_is_alive.0, movement, pos_and_is_alive.1);
            } else if registry.has_component::<SentinelMoveComponent>(e) {
                let (movement, ) = get_components_unwrap!(registry, e, SentinelMoveComponent);
                let pos_and_is_alive = get_target_lifelihood_and_position(registry, movement.target);
                enemy_pos = sentinel_move(enemy_pos, pos_and_is_alive.0, movement, pos_and_is_alive.1);
            } else if registry.has_component::<BulletMoveComponent>(e) {
                let (movement, ) = get_components_unwrap!(registry, e, BulletMoveComponent);
                enemy_pos = bullet_move(enemy_pos, movement);
            }

            self.collide_player(enemy_pos, &enemy_size, registry);
            registry.add_component(e, PositionComponent { pos: enemy_pos });
        }
    }
}

impl EnemyMovementSystem {
    fn collide_player(&mut self, e_pos: Vec2, e_size: &SizeComponent, registry: &mut Registry) {
        for e in entities_with!(registry, PlayerComponent) {
            let (p_pos, p_size) = get_components_unwrap!(registry,e,PositionComponent,SizeComponent);
            let player_hit = create_box(p_pos.pos, p_size.width as f32, p_size.height as f32);
            let enemy_hit = create_box(e_pos, e_size.width as f32, e_size.height as f32);

            if enemy_hit.rect_inter(&player_hit) {
                self.damage_topic.send_message((e, 1, 1));
            }
        }
    }
}

fn straight_move(enemy_pos: Vec2, target_pos: Vec2, movement: &StraightMoveComponent, is_alive: bool) -> Vec2 {
    let enemy_pos = enemy_pos;
    let mut direction_to_player = (target_pos - enemy_pos).normalized();

    if !is_alive { direction_to_player = direction_to_player * -1.0; };

    enemy_pos + direction_to_player * movement.speed
}

fn spiral_move(enemy_pos: Vec2, target_pos: Vec2, movement: &SpiralMoveComponent, is_alive: bool) -> Vec2 {
    let direction_to_player = (target_pos - enemy_pos).normalized();

    let perp_movement = direction_to_player.perp() * movement.perpendicular_speed;
    let mut par_movement = direction_to_player * movement.parallel_speed;

    if !is_alive { par_movement = par_movement * -1.0; };

    enemy_pos + perp_movement + par_movement
}

fn sentinel_move(enemy_pos: Vec2, target_pos: Vec2, movement: &SentinelMoveComponent, is_alive: bool) -> Vec2 {
    let enemy_pos = enemy_pos;
    let mut direction_to_player = target_pos - enemy_pos;
    let player_distance = direction_to_player.norm();

    if player_distance >= movement.stopping_distance {
        enemy_pos + direction_to_player.normalized() * movement.speed
    } else {
        enemy_pos + direction_to_player.normalized() * -1 as f32 * movement.speed
    }
}

fn bullet_move(enemy_pos: Vec2, movement: &BulletMoveComponent) -> Vec2 {
    enemy_pos + movement.direction.normalized() * movement.speed
}

fn get_target_lifelihood_and_position(registry: &Registry, target: Entity) -> (Vec2, bool) {
    if registry.is_alive(target) {
        let (target_pos, ) = get_components_unwrap!(registry,target, PositionComponent);
        (target_pos.pos, true)
    } else {
        let mut pos = Vec2 { x: 0.0, y: 0.0 };
        for tomb in entities_with!(registry,TombstoneComponent, PositionComponent) {
            let (target_pos, ) = get_components_unwrap!(registry,tomb, PositionComponent);
            pos = target_pos.pos;
        }
        (pos, false)
    }
}