use std::f32::consts::TAU;

use crate::{Abort, DRAW_COLORS, entities_with, get_components_clone_unwrap, get_components_unwrap, has_all_components, int_to_string, rect, Registry, SCREEN_SIZE, text, Topic, trace, Vec2};
use crate::assets::{init_boss, init_fly, init_orbiter, init_shooting_orbiter, init_spitworm, init_sprinter};
use crate::ecs::Entity;
use crate::game::components::{BulletMoveComponent, EnemyComponent, GameManagerComponent, PlayerComponent, PositionComponent};
use crate::game::systems::System;
use crate::utils::{cos, sin};

pub const NB_WAVES: u8 = 9;
const WAVES: [Wave; NB_WAVES as usize] = [
    Wave { nb_sprinter: 0, nb_fly: 0, nb_spitworm: 0, nb_boss: 1, nb_orbiter: 0, nb_shooting_orbiter: 0, radius: 100.0 },
    Wave { nb_sprinter: 5, nb_fly: 0, nb_spitworm: 0, nb_boss: 0, nb_orbiter: 0, nb_shooting_orbiter: 0, radius: 60.0 },
    Wave { nb_sprinter: 10, nb_fly: 0, nb_spitworm: 0, nb_boss: 0, nb_orbiter: 0, nb_shooting_orbiter: 0, radius: 60.0 },
    Wave { nb_sprinter: 0, nb_fly: 5, nb_spitworm: 0, nb_boss: 0, nb_orbiter: 0, nb_shooting_orbiter: 0, radius: 50.0 },
    Wave { nb_sprinter: 0, nb_fly: 10, nb_spitworm: 0, nb_boss: 0, nb_orbiter: 0, nb_shooting_orbiter: 0, radius: 50.0 },
    Wave { nb_sprinter: 0, nb_fly: 0, nb_spitworm: 5, nb_boss: 0, nb_orbiter: 0, nb_shooting_orbiter: 0, radius: 60.0 },
    Wave { nb_sprinter: 5, nb_fly: 0, nb_spitworm: 3, nb_boss: 0, nb_orbiter: 0, nb_shooting_orbiter: 0, radius: 60.0 },
    Wave { nb_sprinter: 5, nb_fly: 5, nb_spitworm: 5, nb_boss: 0, nb_orbiter: 0, nb_shooting_orbiter: 0, radius: 80.0 },
    Wave { nb_sprinter: 0, nb_fly: 0, nb_spitworm: 0, nb_boss: 1, nb_orbiter: 0, nb_shooting_orbiter: 0, radius: 60.0 },
];

pub struct EnemyWavesSystem {
    pub next_wave: u8,
    pub score_topic: Topic<i32>,
}

#[derive(Clone, Copy)]
struct Wave {
    nb_sprinter: u8,
    nb_fly: u8,
    nb_spitworm: u8,
    nb_boss: u8,
    nb_orbiter: u8,
    nb_shooting_orbiter: u8,
    radius: f32,
}

impl Wave {
    fn num_enemies(self) -> u8 {
        self.nb_fly + self.nb_spitworm + self.nb_sprinter + self.nb_boss + self.nb_orbiter + self.nb_shooting_orbiter
    }
}

impl EnemyWavesSystem {
    fn spawn_wave(&mut self, registry: &mut Registry, wave: Wave, mouvement_target: Entity, attack_target: Entity) {
        let num_enemies = wave.num_enemies();
        let spawn_radius = wave.radius;
        let dtheta = TAU / num_enemies as f32;

        let (player_pos, ) = get_components_unwrap!(registry, mouvement_target, PositionComponent);
        let player_pos = player_pos.pos;

        for i in 0..wave.nb_sprinter {
            let pos = get_enemy_pos(i as f32, 0.0, wave.nb_sprinter as f32, player_pos, spawn_radius);

            let e = registry.new_entity();
            init_sprinter(registry, e, attack_target);
            registry.add_component(e, PositionComponent { pos }).abort();
        }
        for i in 0..wave.nb_fly {
            let pos = get_enemy_pos(i as f32, dtheta, wave.nb_fly as f32, player_pos, spawn_radius);

            let e = registry.new_entity();
            init_fly(registry, e, mouvement_target);
            registry.add_component(e, PositionComponent { pos }).abort();
        }
        for i in 0..wave.nb_spitworm {
            let pos = get_enemy_pos(i as f32, 2.0 * dtheta, wave.nb_spitworm as f32, player_pos, spawn_radius);

            let e = registry.new_entity();
            init_spitworm(registry, e, mouvement_target, attack_target);
            registry.add_component(e, PositionComponent { pos }).abort();
        }
        for i in 0..wave.nb_boss {
            let pos = get_enemy_pos(i as f32, 3.0 * dtheta, wave.nb_boss as f32, player_pos, spawn_radius);

            let e = registry.new_entity();
            init_boss(registry, e, mouvement_target, attack_target);
            registry.add_component(e, PositionComponent { pos }).abort();
            self.spawn_wave(registry, Self::get_boss_wave(), e, mouvement_target);
        }
        for i in 0..wave.nb_orbiter {
            let pos = get_enemy_pos(i as f32, 4.0 * dtheta, wave.nb_orbiter as f32, player_pos, spawn_radius);

            let e = registry.new_entity();
            init_orbiter(registry, e, mouvement_target);
            registry.add_component(e, PositionComponent { pos }).abort();
        }
        for i in 0..wave.nb_shooting_orbiter {
            let pos = get_enemy_pos(i as f32, 4.0 * dtheta, wave.nb_shooting_orbiter as f32, player_pos, spawn_radius);

            let e = registry.new_entity();
            init_shooting_orbiter(registry, e, mouvement_target, attack_target);
            registry.add_component(e, PositionComponent { pos }).abort();
        }
    }

    fn get_boss_wave() -> Wave {
        Wave {
            nb_sprinter: 0,
            nb_fly: 0,
            nb_spitworm: 0,
            nb_boss: 0,
            nb_orbiter: 15,
            nb_shooting_orbiter: 4,
            radius: 50.0,
        }
    }
}


impl System for EnemyWavesSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let num_enemies = entities_with!(registry, EnemyComponent).iter().count() - entities_with!(registry, BulletMoveComponent).iter().count();
        if num_enemies == 0 {
            self.score_topic.send_message(100);
            if self.next_wave == 0 || self.next_wave - 1 <= NB_WAVES {
                self.next_wave += 1;
            }
            for game_manager_entity in entities_with!(registry, GameManagerComponent) {
                let (mut game_manager, ) = get_components_clone_unwrap!(registry, game_manager_entity, GameManagerComponent);
                game_manager.current_wave = self.next_wave;
                registry.add_component(game_manager_entity, game_manager);

                if self.next_wave - 1 < NB_WAVES {
                    for player_entity in entities_with!(registry, PlayerComponent, PositionComponent) {
                        self.spawn_wave(registry, WAVES[(self.next_wave - 1) as usize], player_entity, player_entity);
                    }
                }
            }
        }
        let bottom_screen = SCREEN_SIZE as i32 - 8;
        unsafe { *DRAW_COLORS = 0x0002; }
        rect(0, bottom_screen, 160, 8);
        unsafe { *DRAW_COLORS = 0x0023; }
        let wave_text = "Wave:";
        text(wave_text, 0, bottom_screen);
        let wave_text_len = wave_text.len() as i32;
        text(int_to_string(self.next_wave as i32), 8 * wave_text_len, bottom_screen);

        let enemies_text = "Enemies:";
        let spacing = 5;
        text(enemies_text, 8 * (wave_text_len + spacing), bottom_screen);
        let enemies_text_len = enemies_text.len() as i32;
        text(int_to_string(num_enemies as i32), 8 * (enemies_text_len + wave_text_len + spacing) as i32, bottom_screen);
    }
}

fn get_enemy_pos(i: f32, dtheta: f32, nb_entity: f32, player_pos: Vec2, spawn_radius: f32) -> Vec2 {
    let theta = i * TAU / nb_entity + dtheta;
    let c = player_pos.x + cos(theta) * spawn_radius;
    let s = player_pos.y + sin(theta) * spawn_radius;
    Vec2::new(c, s)
}
