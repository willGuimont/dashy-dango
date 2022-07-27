#![feature(box_syntax)]
#![feature(once_cell)]

use std::any::Any;
use std::lazy::Lazy;
use std::mem::transmute;
use std::sync::{Arc, Mutex};

use ecs_macro::Component;

use wasm4::*;

use crate::abort::Abort;
use crate::ecs::{BaseComponent, Entity, Registry};
use crate::events::{Subscriber, Topic};
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
mod abort;

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
static mut REG: Lazy<Arc<Mutex<Registry>>> = Lazy::new(|| Arc::new(Mutex::new(Registry::new())));

#[derive(Component, Clone)]
struct PositionComponent {
    x: i16,
    y: i16,
}

#[derive(Component, Clone)]
struct HealthComponent {
    hp: i16,
}

#[derive(Component, Clone)]
struct SpeedComponent {
    speed: i16,
}

#[no_mangle]
fn start() {
    let mut registry = unsafe { REG.lock().abort() };

    let e = registry.new_entity();
    registry.add_component(e, PositionComponent { x: 0, y: 0 }).abort();
    registry.add_component(e, HealthComponent { hp: 1 }).abort();
}

#[no_mangle]
fn update() {
    let mut registry = unsafe { REG.lock().abort() };

    for (_, (pos, health)) in entities_with_components!(registry, PositionComponent, HealthComponent) {}

    // mut
    for e in entities_with!(registry, PositionComponent, HealthComponent) {
        let (mut pos, mut health) = get_components_clone_unwrap!(registry, e, PositionComponent, HealthComponent);
        pos.x += 1;
        health.hp = 100;
        add_components!(&mut registry, e, pos, health);
    }

    for (_, (pos, health)) in entities_with_components!(registry, PositionComponent, HealthComponent) {}

    let mut topic: Topic<i32> = Topic::new();
    let mut sub_1 = Subscriber::new();
    let mut sub_2 = Subscriber::new();
    sub_1.follow(&mut topic);
    sub_2.follow(&mut topic);

    topic.send_message(123);
    topic.send_message(456);
    sub_1.pop_message().abort();

    unsafe { *DRAW_COLORS = 2 }
    let hello = camera_conversion(10.0, 10.0);
    let score = 10;
    text(format!("Hello from Rust! {}", score), hello.0, hello.1);

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
