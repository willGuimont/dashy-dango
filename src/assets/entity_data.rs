use std::task::ready;

use crate::{Abort, BLIT_FLIP_X, BLIT_FLIP_Y, Registry, Vec2};
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

pub fn init_spitworm(registry: &mut Registry, e: Entity, mouvement_target: Entity, attack_target: Entity) {
    registry.add_component(e, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(e, EnemyComponent).abort();
    registry.add_component(e, SentinelMoveComponent { speed: 0.2, stopping_distance: 60.0, target: mouvement_target }).abort();
    registry.add_component(e, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &SPITWORM_SPRITE, zindex: 2, is_visible: true }).abort();
    registry.add_component(e, ScoreComponent { score: 10 }).abort();
    registry.add_component(e, ShooterComponent { bullet_speed: 1.0, firing_timeout: 50, firing_delay: 50, bullet_lifespan: 150, firing_distance: 60, target: attack_target });
}

pub fn init_orbiter(registry: &mut Registry, e: Entity, mouvement_target: Entity) {
    let target_pos = registry.get_component::<PositionComponent>(mouvement_target).abort().pos;

    registry.add_component(e, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(e, EnemyComponent).abort();
    registry.add_component(e, OrbitingMoveComponent { last_pos: target_pos, target: mouvement_target, perp_speed: 0.9 });
    registry.add_component(e, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &FLY_SPRITE, zindex: 2, is_visible: true }).abort();
    registry.add_component(e, ScoreComponent { score: 10 }).abort();
}

pub fn init_shooting_orbiter(registry: &mut Registry, e: Entity, mouvement_target: Entity, attack_target: Entity) {
    let target_pos = registry.get_component::<PositionComponent>(mouvement_target).abort().pos;

    registry.add_component(e, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(e, EnemyComponent).abort();
    registry.add_component(e, OrbitingMoveComponent { last_pos: target_pos, target: mouvement_target, perp_speed: 0.5 });
    registry.add_component(e, ShooterComponent { bullet_speed: 1.0, firing_timeout: 50, firing_delay: 100, bullet_lifespan: 150, firing_distance: 60, target: attack_target });
    registry.add_component(e, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &ORBITING_SHOOTER_SPRITE, zindex: 3, is_visible: true }).abort();
    registry.add_component(e, ScoreComponent { score: 10 }).abort();
}

pub fn init_boss(registry: &mut Registry, e: Entity, mouvement_target: Entity, attack_target: Entity) {
    registry.add_component(e, HealthComponent { hp: 3, timeout: 0, hit_delay: 20 });
    registry.add_component(e, EnemyComponent);
    registry.add_component(e, StraightMoveComponent { speed: 0.2, target: mouvement_target });
    registry.add_component(e, SizeComponent { width: 8, height: 16 }).abort();
    registry.add_component(e, ScoreComponent { score: 10 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &BOSS_SPRITE, zindex: 3, is_visible: true });
    registry.add_component(e, BossComponent);
}
