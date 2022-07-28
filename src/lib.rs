#![feature(box_syntax)]
#![feature(once_cell)]

use std::any::Any;
use std::sync::{Arc, Mutex};

use ecs_macro::Component;
use once_cell::sync::Lazy;

use wasm4::*;

use crate::abort::Abort;
use crate::ecs::{BaseComponent, Entity, Registry};
use crate::events::{Subscriber, Topic};
use crate::game::camera_component::CameraComponent;
use crate::game::components::position_component::PositionComponent;
use crate::game::dash_component::DashComponent;
use crate::game::draw_system::draw_entity;
use crate::game::gamepad_component::GamepadComponent;
use crate::game::health_component::HealthComponent;
use crate::game::move_component::MoveComponent;
use crate::game::move_system::process_player_movement;
use crate::math_utils::*;
use crate::utils::gamepad_utils;

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

const PLAYER_BASE_SPEED: i16 = 2;
const PLAYER_BASE_DASH: i16 = 5;
static mut REGISTRY: Lazy<Arc<Mutex<Registry>>> = Lazy::new(|| Arc::new(Mutex::new(Registry::new())));


#[no_mangle]
fn start() {
    unsafe { *DRAW_COLORS = 2 }

    let mut registry = unsafe { REGISTRY.lock().abort() };
    create_player(&mut registry);

    for i in 0..10 {
        let e = registry.new_entity();
        registry.add_component(e, PositionComponent { x: (i * 8) as f32, y: (i * 8) as f32 }).unwrap();
        registry.add_component(e, HealthComponent { hp: i }).abort();
    }
}

#[no_mangle]
fn update() {
    let mut registry = unsafe { REGISTRY.lock().abort() };

    let mut topic: Topic<i32> = Topic::new();
    let mut sub_1 = Subscriber::new();
    let mut sub_2 = Subscriber::new();
    sub_1.follow(&mut topic);
    sub_2.follow(&mut topic);

    topic.send_message(123);
    topic.send_message(456);
    sub_1.pop_message().abort();

    process_player_movement(&mut registry);
    draw_entity(&registry);
}


fn create_player(registry: &mut Registry) {
    let gamepad = GAMEPAD1;
    let player = registry.new_entity();
    registry.add_component(player, PositionComponent { x: 0.0, y: 0.0 }).abort();
    registry.add_component(player, GamepadComponent { gamepad }).abort();
    registry.add_component(player, MoveComponent { speed: PLAYER_BASE_SPEED }).abort();
    registry.add_component(player, DashComponent { dash: PLAYER_BASE_DASH, timeout: 1 }).abort();
    registry.add_component(player, CameraComponent {}).abort();
}
