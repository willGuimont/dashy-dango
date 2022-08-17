use crate::{Abort, Registry};
use crate::assets::*;
use crate::ecs::Entity;
use crate::game::components::*;

pub fn init_sprinter(registry: &mut Registry, e: Entity, target: Entity) {
    registry.add_component(e, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(e, EnemyComponent).abort();
    registry.add_component(e, StraightMoveComponent { speed: 0.5, target }).abort();
    registry.add_component(e, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &ROBOT_SPRITE, zindex: 2, is_visible: true }).abort();
    registry.add_component(e, ScoreComponent { score: 10 }).abort();
}

pub fn init_fly(registry: &mut Registry, e: Entity, target: Entity) {
    registry.add_component(e, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(e, EnemyComponent).abort();
    registry.add_component(e, SpiralMoveComponent { perpendicular_speed: 0.9, parallel_speed: 0.1, target }).abort();
    registry.add_component(e, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &FLY_SPRITE, zindex: 2, is_visible: true }).abort();
    registry.add_component(e, ScoreComponent { score: 10 }).abort();
}

pub fn init_spitworm(registry: &mut Registry, e: Entity, target: Entity) {
    registry.add_component(e, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(e, EnemyComponent).abort();
    registry.add_component(e, SentinelMoveComponent { speed: 0.2, stopping_distance: 50.0, target }).abort();
    registry.add_component(e, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &SPITWORM_SPRITE, zindex: 2, is_visible: true }).abort();
    registry.add_component(e, ScoreComponent { score: 10 }).abort();
    registry.add_component(e, ShooterComponent { bullet_speed: 1.0, firing_timeout: 50, firing_delay: 50, bullet_lifespan: 100, firing_distance: 50, target });
}
