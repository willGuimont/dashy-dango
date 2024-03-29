use crate::{Abort, entities_with, get_components_clone_unwrap, get_components_unwrap, has_all_components, Registry, Vec2};
use crate::assets::BULLET_SPRITE;
use crate::ecs::Entity;
use crate::game::components::{BulletMoveComponent, EnemyComponent, HealthComponent, PositionComponent, ScoreComponent, ShooterComponent, SizeComponent, SpriteComponent};
use crate::game::components::ttl_component::TTLComponent;
use crate::game::systems::System;

pub struct EnemyAttackSystem;

impl System for EnemyAttackSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry, EnemyComponent) {
            if registry.has_component::<ShooterComponent>(e)
            {
                shoot_attack(registry, e);
            }
        }
    }
}

fn shoot_attack(registry: &mut Registry, e: Entity) {
    let (mut shoot, enemy_pos) = get_components_clone_unwrap!(registry, e, ShooterComponent, PositionComponent);
    if registry.is_alive(shoot.target) {
        let (target_pos, ) = get_components_unwrap!(registry,shoot.target, PositionComponent);
        let enemy_pos = enemy_pos.pos;
        let direction_to_player = target_pos.pos - enemy_pos;
        let player_distance = direction_to_player.norm();

        if shoot.firing_timeout <= 0 && player_distance <= shoot.firing_distance as f32 {
            create_bullet(registry, direction_to_player.normalized(), shoot.bullet_speed, shoot.bullet_lifespan, enemy_pos.x, enemy_pos.y);
            shoot.firing_timeout = shoot.firing_delay;
            registry.add_component(e, shoot);
        } else {
            shoot.firing_timeout -= 1;
            registry.add_component(e, shoot);
        }
    }
}

fn create_bullet(registry: &mut Registry, direction: Vec2, speed: f32, bullet_lifespan: i16, bullet_x: f32, bullet_y: f32) {
    let bullet = registry.new_entity();
    registry.add_component(bullet, EnemyComponent {}).abort();
    registry.add_component(bullet, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(bullet, BulletMoveComponent { speed, direction }).abort();
    registry.add_component(bullet, SizeComponent { width: 2, height: 2 }).abort();
    registry.add_component(bullet, SpriteComponent { sprite: &BULLET_SPRITE, zindex: 3, is_visible: true }).abort();
    registry.add_component(bullet, TTLComponent { ttl: bullet_lifespan }).abort();
    registry.add_component(bullet, PositionComponent { pos: Vec2 { x: bullet_x, y: bullet_y } }).abort();
    registry.add_component(bullet, ScoreComponent { score: 10 });
}
