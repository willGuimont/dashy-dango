use std::f32::consts::TAU;

use crate::{Abort, entities_with, has_all_components, REFRESH_RATE, Registry};
use crate::ecs::Entity;
use crate::game::components::{EnemyComponent, PositionComponent};
use crate::game::systems::System;

pub struct EnemyWavesSystem {
    delay_between_waves: i32,
    timeout: i32,
}

impl EnemyWavesSystem {
    pub fn new(delay_between_waves: i32) -> Self {
        EnemyWavesSystem { delay_between_waves, timeout: 0 }
    }

    fn spawn_new_wave(&mut self, registry: &mut Registry) {
        self.timeout = self.delay_between_waves;

        let num_enemies = 10;
        let spawn_radius: f32 = 50.0;
        for i in 0..num_enemies {
            let theta = (i as f32) / (num_enemies as f32) * 2.0 * TAU;
            let c = theta.cos() * spawn_radius;
            let s = theta.sin() * spawn_radius;
            let e = registry.new_entity();
            registry.add_component(e, PositionComponent { x: c, y: s }).abort();

            let ttl: i32 = i * REFRESH_RATE;
            registry.add_component(e, EnemyComponent { time_to_live: ttl }).abort();
        }
    }
}

impl System for EnemyWavesSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let num_enemies = entities_with!(registry, EnemyComponent).iter().count();
        if self.timeout <= 0 || num_enemies == 0 {
            self.spawn_new_wave(registry);
        }
    }
}
