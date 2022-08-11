use crate::{DRAW_COLORS, Registry, Subscriber, text};
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
        self.score_decrease();
        self.read_event_queue();

        unsafe { *DRAW_COLORS = 0x0023; }
        text(int_to_string(self.score), 76, 0);
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

