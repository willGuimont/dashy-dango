use crate::{Registry, Subscriber, tone, TONE_TRIANGLE};
use crate::game::systems::System;

const BASE_FREQUENCY: u32 = 280;

#[derive(Copy, Clone)]
pub enum SoundEvent {
    Kill(u32),
    TakeDamage,
    Die,
    Win,
    Dash,
}

pub struct SoundSystem {
    pub sound_queue: Subscriber<SoundEvent>,
}

impl System for SoundSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        self.play_sounds();
    }
}

impl SoundSystem {
    fn play_sounds(&mut self) {
        while let Some(soundEvent) = self.sound_queue.pop_message() {
            let sound = get_sound(soundEvent);
            let (freq, duration) = make_sound(sound.0, sound.1, 0, 0, sound.2, 16);
            tone(freq, duration, 100, TONE_TRIANGLE);
        }
    }
}

//Function to create sound from variables. Code taken from Peterhellberg https://github.com/aduros/wasm4/discussions/8#discussioncomment-1348839
fn make_sound(freq1: u32, freq2: u32, ack: u32, dec: u32, sus: u32, rel: u32) -> (u32, u32) {
    let freq = freq1 | freq2 << 16;
    let duration = ack << 24 | dec << 16 | sus | rel << 8;
    (freq, duration)
}

fn get_sound(sound_event: SoundEvent) -> (u32, u32, u32) {
    match sound_event {
        SoundEvent::Kill(mut num) => {
            if num > 8 { num = 8; }
            (BASE_FREQUENCY * num, BASE_FREQUENCY * 2 * num, 10)
        }
        SoundEvent::TakeDamage => { (BASE_FREQUENCY, BASE_FREQUENCY / 2, 10) }
        SoundEvent::Die => { (BASE_FREQUENCY, BASE_FREQUENCY / 4, 60) }
        SoundEvent::Win => { (BASE_FREQUENCY, BASE_FREQUENCY * 4, 60) }
        SoundEvent::Dash => { (0, 0, 0) }
    }
}