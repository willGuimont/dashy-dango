use crate::{Abort, create_box, entities_with, entities_with_components, get_components_clone_unwrap, get_components_unwrap, has_all_components, Registry, trace, Vec2};
use crate::assets::GRASS_SPRITE;
use crate::ecs::Entity;
use crate::game::components::{EnemyComponent, HealthComponent, PlayerComponent, PositionComponent, SizeComponent, SpriteComponent};
use crate::game::components::bullet_move_component::BulletMoveComponent;
use crate::game::components::enemy_attack_components::shooter_component::ShooterComponent;
use crate::game::components::sentinel_move_component::SentinelMoveComponent;
use crate::game::components::spiral_move_component::SpiralMoveComponent;
use crate::game::components::straight_move_component::StraightMoveComponent;
use crate::game::components::ttl_component::TTLComponent;
use crate::game::systems::System;

pub struct EnemySystem;

impl System for EnemySystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let (_, (_, player_pos)) = entities_with_components!(registry, PlayerComponent, PositionComponent).next().abort();
        let player_pos = player_pos.pos;
        for e in entities_with!(registry, EnemyComponent) {
            if registry.has_component::<StraightMoveComponent>(e) {
                straight_move(registry, e, player_pos);
            } else if registry.has_component::<SpiralMoveComponent>(e) {
                spiral_move(registry, e, player_pos);
            } else if registry.has_component::<SentinelMoveComponent>(e) {
                sentinel_move(registry, e, player_pos);
            } else if registry.has_component::<BulletMoveComponent>(e) {
                bullet_move(registry, e, player_pos);
            }
        }
    }
}

fn straight_move(registry: &mut Registry, e: Entity, player_pos: Vec2) {
    let (movement, enemy_pos, enemy_size) = get_components_clone_unwrap!(registry, e, StraightMoveComponent, PositionComponent, SizeComponent);
    let enemy_pos = enemy_pos.pos;
    let direction_to_player = (player_pos - enemy_pos).normalized();

    let enemy_pos = enemy_pos + direction_to_player * movement.speed;

    collide_player(enemy_pos, &enemy_size, registry);
    execute_enemy_attack(registry, e, player_pos);
    registry.add_component(e, PositionComponent { pos: enemy_pos });
}

fn spiral_move(registry: &mut Registry, e: Entity, player_pos: Vec2) {
    let (movement, enemy_pos, enemy_size) = get_components_clone_unwrap!(registry, e, SpiralMoveComponent, PositionComponent, SizeComponent);
    let enemy_pos = enemy_pos.pos;
    let direction_to_player = (player_pos - enemy_pos).normalized();

    let perp_movement = direction_to_player.perp() * movement.perpendicular_speed;
    let par_movement = direction_to_player * movement.parallel_speed;
    let enemy_pos = enemy_pos + perp_movement + par_movement;

    collide_player(enemy_pos, &enemy_size, registry);
    execute_enemy_attack(registry, e, player_pos);
    registry.add_component(e, PositionComponent { pos: enemy_pos });
}

fn sentinel_move(registry: &mut Registry, e: Entity, player_pos: Vec2) {
    let (movement, enemy_pos, enemy_size) = get_components_clone_unwrap!(registry, e, SentinelMoveComponent, PositionComponent, SizeComponent);
    let enemy_pos = enemy_pos.pos;
    let direction_to_player = (player_pos - enemy_pos);
    let player_distance = direction_to_player.norm();

    if player_distance >= movement.shooting_distance {
        let enemy_pos = enemy_pos + direction_to_player.normalized() * movement.speed;
        collide_player(enemy_pos, &enemy_size, registry);
        registry.add_component(e, PositionComponent { pos: enemy_pos });
    } else {
        let enemy_pos = enemy_pos + direction_to_player.normalized() * -1 as f32 * movement.speed;
        collide_player(enemy_pos, &enemy_size, registry);
        execute_enemy_attack(registry, e, player_pos);
        registry.add_component(e, PositionComponent { pos: enemy_pos });
    }
}

fn bullet_move(registry: &mut Registry, e: Entity, player_pos: Vec2) {
    let (movement, enemy_pos, enemy_size) = get_components_clone_unwrap!(registry, e, BulletMoveComponent, PositionComponent, SizeComponent);
    let enemy_pos = enemy_pos.pos + movement.direction.normalized() * movement.speed;
    collide_player(enemy_pos, &enemy_size, registry);
    execute_enemy_attack(registry, e, player_pos);
    registry.add_component(e, PositionComponent { pos: enemy_pos });
}

fn execute_enemy_attack(registry: &mut Registry, e: Entity, player_pos: Vec2) {
    if has_all_components!(registry, e, ShooterComponent) {
        shoot_attack(registry, e, player_pos);
    }
}

fn shoot_attack(registry: &mut Registry, e: Entity, player_pos: Vec2) {
    let (mut shoot, enemy_pos) = get_components_clone_unwrap!(registry, e, ShooterComponent, PositionComponent);
    let enemy_pos = enemy_pos.pos;
    let direction_to_player = (player_pos - enemy_pos).normalized();

    if shoot.firing_timeout <= 0 {
        create_bullet(registry, direction_to_player, shoot.bullet_speed, shoot.bullet_lifespan, enemy_pos.x, enemy_pos.y);
        shoot.firing_timeout += shoot.firing_delay;
        registry.add_component(e, shoot);
    } else {
        shoot.firing_timeout -= 1;
        registry.add_component(e, shoot);
    }
}

fn collide_player(e_pos: Vec2, e_size: &SizeComponent, registry: &mut Registry) {
    for e in entities_with!(registry, PlayerComponent) {
        let (mut health, p_pos, p_size) = get_components_clone_unwrap!(registry,e,HealthComponent,PositionComponent,SizeComponent);
        let player_hit = create_box(p_pos.pos, p_size.width as f32, p_size.height as f32);
        let enemy_hit = create_box(e_pos, e_size.width as f32, e_size.height as f32);
        if health.timeout > 0 {
            health.timeout -= 1;
            registry.add_component(e, health);
        } else if health.timeout <= 0 && enemy_hit.rect_inter(&player_hit) {
            health.hp -= 1;
            health.timeout += health.hit_delay;
            registry.add_component(e, health);
        }
    }
}

fn create_bullet(registry: &mut Registry, direction: Vec2, speed: f32, bullet_lifespan: i16, bullet_x: f32, bullet_y: f32) {
    let bullet = registry.new_entity();
    registry.add_component(bullet, EnemyComponent {}).abort();
    registry.add_component(bullet, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(bullet, BulletMoveComponent { speed, direction }).abort();
    registry.add_component(bullet, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(bullet, SpriteComponent { sprite: &GRASS_SPRITE }).abort();
    registry.add_component(bullet, TTLComponent { ttl: bullet_lifespan }).abort();
    registry.add_component(bullet, PositionComponent { pos: Vec2 { x: bullet_x, y: bullet_y } });
}
