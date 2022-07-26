use std::any::Any;

use ecs_macro::Component;

use wasm4::*;

use crate::ecs::{BaseComponent, Registry};
use crate::utils::keyboard_utils;

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod math_utils;
mod ecs;
mod game;
mod utils;
mod events;
mod assets;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

static mut PLAYER_X: f32 = 76.0;
static mut PLAYER_Y: f32 = 76.0;
const CENTER: (f32, f32) = (76.0, 76.0);

#[derive(Component, Debug)]
struct PositionComponent {
    x: i16,
    y: i16,
}

#[derive(Component, Debug)]
struct HealthComponent {
    hp: i16,
}

#[derive(Component, Debug)]
struct SpeedComponent {
    speed: i16,
}

#[no_mangle]
fn update() {
    let mut registry = Registry::new();

    for i in 0..10 {
        let e = registry.new_entity();
        registry.add_component(e, PositionComponent { x: i, y: i });
        if i % 2 == 0 {
            registry.add_component(e, HealthComponent { hp: i });
        }
    }

    trace("Listing entities with components");
    for (pos, health) in entities_with_components!(registry, PositionComponent, HealthComponent) {
        trace(format!("{:?}, {:?}", pos, health));
    }

    unsafe { *DRAW_COLORS = 2 }
    let hello = camera_conversion(10.0, 10.0);
    text("Hello from Rust!", hello.0, hello.1);

    let gamepad = unsafe { *GAMEPAD1 };
    let direction = keyboard_utils::gamepad_to_vec(gamepad);
    unsafe {
        PLAYER_X += direction.0;
        PLAYER_Y += direction.1;
    }

    unsafe {
        let me = camera_conversion(PLAYER_X, PLAYER_Y);
        blit(&SMILEY, me.0, me.1, 8, 8, BLIT_1BPP);
    }

    unsafe { *DRAW_COLORS = 4 }
    let other = camera_conversion(80.0, 80.0);
    blit(&SMILEY, other.0, other.1, 8, 8, BLIT_1BPP);

    let press = camera_conversion(16.0, 90.0);
    text("Press X to dash", press.0, press.1);
}

fn camera_conversion(x: f32, y: f32) -> (i32, i32) {
    unsafe { ((x - PLAYER_X + CENTER.0) as i32, (y - PLAYER_Y + CENTER.1) as i32) }
}
