use crate::{DRAW_COLORS, Registry, Subscriber, text};
use crate::game::systems::System;

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

//Taken from https://doc.rust-lang.org/src/alloc/string.rs.html#2517-2536, recoded to avoid importing it and reduce cart size
fn int_to_string(num: i32) -> String {
    let mut buf = String::with_capacity(4);
    if num < 0 {
        buf.push('-');
    }
    let mut n: u32 = if num < 0 { -num as u32 } else { num as u32 };

    if n >= 10 {
        if n >= 100 {
            if n >= 1000 {
                if n >= 10000 {
                    buf.push((b'0' + ((n / 10000) as u8)) as char);
                    n %= 10000;
                }
                buf.push((b'0' + ((n / 1000) as u8)) as char);
                n %= 1000;
            }
            buf.push((b'0' + ((n / 100) as u8)) as char);
            n %= 100;
        }
        buf.push((b'0' + ((n / 10) as u8)) as char);
        n %= 10;
    }
    buf.push((b'0' + (n as u8)) as char);
    buf
}
