use crate::{Registry, Subscriber, tone, TONE_TRIANGLE};
use crate::game::systems::System;

pub struct SoundSystem {
    pub sound_queue: Subscriber<(u32, u32, u32)>,
}

impl System for SoundSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        self.play_sounds();
    }
}

impl SoundSystem {
    fn play_sounds(&mut self) {
        while let Some(sound) = self.sound_queue.pop_message() {
            let (freq, duration) = make_sound(sound.0, sound.1, 0, 0, sound.2, 16);
            tone(freq, duration, 100, TONE_TRIANGLE);
        }
    }
}

//Function to create sound from variables. Code taken from Petehellberg https://github.com/aduros/wasm4/discussions/8#discussioncomment-1348839
fn make_sound(freq1: u32, freq2: u32, ack: u32, dec: u32, sus: u32, rel: u32) -> (u32, u32) {
    let freq = freq1 | freq2 << 16;
    let duration = ack << 24 | dec << 16 | sus | rel << 8;
    (freq, duration)
}