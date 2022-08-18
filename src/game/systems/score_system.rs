use crate::{Abort, DRAW_COLORS, entities_with, entities_with_components, get_components_clone_unwrap, has_all_components, Registry, Subscriber, text};
use crate::ecs::Entity;
use crate::game::components::GameManagerComponent;
use crate::game::systems::System;
use crate::utils::int_to_string;

pub struct ScoreSystem {
    pub score: i32,
    pub decrease_timer: u8,
    pub score_decrease_speed: u8,
    pub event_queue: Subscriber<i32>,
}

impl System for ScoreSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for game_manager_entity in entities_with!(registry, GameManagerComponent) {
            let (mut game_manager, ) = get_components_clone_unwrap!(registry, game_manager_entity, GameManagerComponent);
            if !game_manager.game_ended {
                self.score_decrease();
                self.read_event_queue();
                game_manager.score = self.score;
                registry.add_component(game_manager_entity, game_manager);
            }
        }

        unsafe { *DRAW_COLORS = 0x0023; }
        let score_text = "Score:";
        text(score_text, 0, 0);
        text(int_to_string(self.score), 8 * score_text.len() as i32, 0);
    }
}

impl ScoreSystem {
    fn score_decrease(&mut self) {
        if self.decrease_timer <= 0 {
            self.score -= 1;
            self.decrease_timer += self.score_decrease_speed;
        } else {
            self.decrease_timer -= 1;
        }
    }

    fn read_event_queue(&mut self) {
        while let Some(message) = self.event_queue.pop_message() {
            self.score += message;
        }
    }
}

