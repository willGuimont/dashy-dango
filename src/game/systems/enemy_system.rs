use crate::{Abort, entities_with, get_components_clone_unwrap, has_all_components, Registry};
use crate::ecs::Entity;
use crate::game::components::EnemyComponent;
use crate::game::systems::System;

pub struct EnemySystem {}

impl EnemySystem {
    pub fn new() -> Self {
        EnemySystem {}
    }
}

impl System for EnemySystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let mut to_destroy = Vec::new();
        for e in entities_with!(registry, EnemyComponent) {
            let (mut enemy, ) = get_components_clone_unwrap!(registry, e, EnemyComponent);

            if enemy.time_to_live <= 0 {
                to_destroy.push(e);
            } else {
                enemy.time_to_live -= 1;
            }
            registry.add_component(e, enemy).abort();
        }
        for e in to_destroy {
            registry.mark_to_destroy(e);
        }
    }
}
