use crate::{Abort, entities_with, entities_with_components, get_components_clone_unwrap, get_components_unwrap, has_all_components, Registry};
use crate::ecs::Entity;
use crate::game::components::{EnemyComponent, PlayerComponent, PositionComponent};
use crate::game::systems::System;

pub struct EnemySystem {}

impl EnemySystem {
    pub fn new() -> Self {
        EnemySystem {}
    }
}


impl System for EnemySystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let (_, (_, player_pos)) = entities_with_components!(registry, PlayerComponent, PositionComponent).next().abort();
        let player_pos = player_pos.to_vec();
        for e in entities_with!(registry, EnemyComponent) {
            let (enemy, enemy_pos, ) = get_components_clone_unwrap!(registry, e, EnemyComponent, PositionComponent);
            let enemy_pos = enemy_pos.to_vec();
            let direction_to_player = (player_pos - enemy_pos).normalized();

            let enemy_pos = enemy_pos + direction_to_player * enemy.speed;
            registry.add_component(e, PositionComponent::from_vec(enemy_pos));
        }
    }
}
