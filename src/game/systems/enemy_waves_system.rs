use std::f32::consts::{PI, TAU};

use crate::{Abort, entities_with, entities_with_components, get_components_unwrap, has_all_components, Registry, trace, Vec2};
use crate::assets::{FLY_SPRITE, GRASS_SPRITE, init_fly, init_spitworm, init_sprinter, SPITWORM_SPRITE};
use crate::ecs::Entity;
use crate::game::components::{EnemyComponent, HealthComponent, PlayerComponent, PositionComponent, SizeComponent, SpiralMoveComponent, SpriteComponent, StraightMoveComponent};
use crate::game::components::enemy_attack_components::shooter_component::ShooterComponent;
use crate::game::components::sentinel_move_component::SentinelMoveComponent;
use crate::game::systems::System;

const NB_WAVES: u8 = 6;
const WAVES: [Wave; NB_WAVES as usize] = [
    Wave { nb_sprinter: 5, nb_fly: 0, nb_spitworm: 0 },
    Wave { nb_sprinter: 10, nb_fly: 0, nb_spitworm: 0 },
    Wave { nb_sprinter: 0, nb_fly: 5, nb_spitworm: 0 },
    Wave { nb_sprinter: 0, nb_fly: 10, nb_spitworm: 0 },
    Wave { nb_sprinter: 0, nb_fly: 0, nb_spitworm: 5 },
    Wave { nb_sprinter: 5, nb_fly: 5, nb_spitworm: 5 },
];

pub struct EnemyWavesSystem {
    pub current_wave: u8,
}

#[derive(Clone, Copy)]
struct Wave {
    nb_sprinter: u8,
    nb_fly: u8,
    nb_spitworm: u8,
}

impl Wave {
    fn num_enemies(self) -> u8 {
        self.nb_fly + self.nb_spitworm + self.nb_sprinter
    }
}

impl EnemyWavesSystem {
    fn spawn_wave(&mut self, registry: &mut Registry, wave: Wave, player_pos: Vec2) {
        trace(self.current_wave.to_string());
        let num_enemies = wave.num_enemies();
        let spawn_radius = 50.0;
        let dtheta = TAU / num_enemies as f32;

        for i in 0..wave.nb_sprinter {
            let theta = i as f32 * TAU / wave.nb_sprinter as f32;
            let c = player_pos.x + theta.cos() * spawn_radius;
            let s = player_pos.y + theta.sin() * spawn_radius;
            let pos = Vec2::new(c, s);

            summon_sprinter(registry, pos);
        }
        for i in 0..wave.nb_fly {
            let theta = i as f32 * TAU / wave.nb_fly as f32 + dtheta;
            let c = player_pos.x + theta.cos() * spawn_radius;
            let s = player_pos.y + theta.sin() * spawn_radius;
            let pos = Vec2::new(c, s);

            summon_fly(registry, pos);
        }
        for i in 0..wave.nb_spitworm {
            let theta = i as f32 * TAU / wave.nb_spitworm as f32 + (2.0 * dtheta);
            let c = player_pos.x + theta.cos() * spawn_radius;
            let s = player_pos.y + theta.sin() * spawn_radius;
            let pos = Vec2::new(c, s);

            summon_spitworm(registry, pos);
        }
    }
}

impl System for EnemyWavesSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let num_enemies = entities_with!(registry, EnemyComponent).iter().count();
        if num_enemies == 0 {
            let (_, (_, player_pos)) = entities_with_components!(registry, PlayerComponent, PositionComponent).next().abort();
            let player_pos = player_pos.pos;
            self.spawn_wave(registry, WAVES[self.current_wave as usize], player_pos);
            self.current_wave += 1;
            if self.current_wave >= NB_WAVES {
                //Implement game winning
                self.current_wave -= 1;
            }
        }
    }
}

fn summon_sprinter(registry: &mut Registry, pos: Vec2) {
    let e = registry.new_entity();
    init_sprinter(registry, e);
    registry.add_component(e, PositionComponent { pos }).abort();
}

fn summon_fly(registry: &mut Registry, pos: Vec2) {
    let e = registry.new_entity();
    init_fly(registry, e);
    registry.add_component(e, PositionComponent { pos }).abort();
}

fn summon_spitworm(registry: &mut Registry, pos: Vec2) {
    let e = registry.new_entity();
    init_spitworm(registry, e);
    registry.add_component(e, PositionComponent { pos }).abort();
}