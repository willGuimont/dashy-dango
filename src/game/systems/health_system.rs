use crate::{Abort, entities_with, entities_with_components, get_components, get_components_clone_unwrap, get_components_unwrap, has_all_components, Registry, Subscriber, Topic, update};
use crate::ecs::Entity;
use crate::game::components::{DashComponent, GameManagerComponent, HealthComponent, PlayerComponent, ScoreComponent};
use crate::game::systems::System;

pub struct HealthSystem {
    pub event_queue: Subscriber<(Entity, i32, i32)>,
    pub score_topic: Topic<i32>,
}

impl System for HealthSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        self.deal_damage(registry);
        self.reduce_hit_timeout(registry);
    }
}

impl HealthSystem {
    fn reduce_hit_timeout(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry,HealthComponent) {
            let (mut health, ) = get_components_clone_unwrap!(registry,e,HealthComponent);
            if health.timeout > 0 {
                health.timeout -= 1;
                registry.add_component(e, health);
            }
        }
    }

    fn deal_damage(&mut self, registry: &mut Registry) {
        while let Some((e, damage, score_multiplier)) = self.event_queue.pop_message() {
            if registry.has_component::<HealthComponent>(e) {
                let (health, ) = get_components_clone_unwrap!(registry,e, HealthComponent);
                if registry.has_component::<PlayerComponent>(e) {
                    self.deal_damage_player(registry, e, health);
                } else {
                    self.deal_damage_enemy(registry, e, health, score_multiplier);
                }
            }
        }
    }

    fn deal_damage_enemy(&mut self, registry: &mut Registry, e: Entity, mut health: HealthComponent, score_multiplier: i32) {
        if health.timeout == 0 {
            health.hp -= 1;
            health.timeout += health.hit_delay;

            if health.hp == 0 {
                let score = registry.get_component::<ScoreComponent>(e).abort();
                self.score_topic.send_message(score.score * score_multiplier);
                registry.destroy_entity(e);
            } else {
                registry.add_component(e, health);
            }
        }
    }

    fn deal_damage_player(&mut self, registry: &mut Registry, e: Entity, mut health: HealthComponent) {
        let dash = registry.get_component::<DashComponent>(e).abort();
        if health.timeout <= 0 && dash.duration <= 0 {
            health.hp -= 1;
            health.timeout += health.hit_delay;
            self.update_game_manager(registry, e, &health);

            if health.hp == 0 {
                //For some reason destroying player causes issue
                registry.add_component(e, health);
                // registry.destroy_entity(e);
            } else {
                registry.add_component(e, health);
            }
        }
    }

    fn update_game_manager(&mut self, registry: &mut Registry, e: Entity, health: &HealthComponent) {
        let (&game_manager_entity, (_, )) = entities_with_components!(registry, GameManagerComponent).next().abort();
        let (mut game_manager, ) = get_components_clone_unwrap!(registry, game_manager_entity, GameManagerComponent);

        game_manager.player_hp = health.hp;
        registry.add_component(game_manager_entity, game_manager);
    }
}
