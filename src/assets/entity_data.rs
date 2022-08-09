use std::path::Component;

use crate::{Abort, BaseComponent, Registry};
use crate::assets::*;
use crate::ecs::Entity;
use crate::game::components::*;

pub fn init_sprinter(registry: &mut Registry, e: Entity) {
    registry.add_component(e, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(e, EnemyComponent).abort();
    registry.add_component(e, StraightMoveComponent { speed: 0.5 }).abort();
    registry.add_component(e, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &GRASS_SPRITE, zindex: 1 }).abort();
}

pub fn init_fly(registry: &mut Registry, e: Entity) {
    registry.add_component(e, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(e, EnemyComponent).abort();
    registry.add_component(e, SpiralMoveComponent { perpendicular_speed: 0.9, parallel_speed: 0.1 }).abort();
    registry.add_component(e, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &FLY_SPRITE, zindex: 1 }).abort();
}

pub fn init_spitworm(registry: &mut Registry, e: Entity) {
    registry.add_component(e, HealthComponent { hp: 1, timeout: 0, hit_delay: 0 }).abort();
    registry.add_component(e, EnemyComponent).abort();
    registry.add_component(e, SentinelMoveComponent { speed: 0.2, stopping_distance: 50.0 }).abort();
    registry.add_component(e, SizeComponent { width: 8, height: 8 }).abort();
    registry.add_component(e, SpriteComponent { sprite: &SPITWORM_SPRITE, zindex: 1 }).abort();
}