#![feature(box_syntax)]
#![feature(once_cell)]

use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use wasm4::*;

use crate::abort::Abort;
use crate::ecs::{BaseComponent, Registry};
use crate::events::{Subscriber, Topic};
use crate::game::components::{CameraComponent, DashComponent, GamepadComponent, HealthComponent, MoveComponent, PositionComponent};
use crate::game::systems::{draw_system, enemy_system, enemy_waves_system, EnemyWavesSystem, move_system};
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

static mut REGISTRY: Lazy<Arc<Mutex<Registry>>> = Lazy::new(|| Arc::new(Mutex::new(Registry::new())));
// TODO move to a component on a game manager entity
static mut WAVES: Lazy<Arc<Mutex<EnemyWavesSystem>>> = Lazy::new(|| Arc::new(Mutex::new(EnemyWavesSystem::new(250))));

#[no_mangle]
fn start() {
    unsafe { *DRAW_COLORS = 2 }

    let registry = &mut unsafe { REGISTRY.lock().abort() };

    let player = registry.new_entity();
    const PLAYER_BASE_SPEED: i16 = 2;
    const PLAYER_BASE_DASH: i16 = 5;
    registry.add_component(player, PositionComponent { x: 0.0, y: 0.0 }).abort();
    registry.add_component(player, GamepadComponent { gamepad: GAMEPAD1 }).abort();
    registry.add_component(player, MoveComponent { speed: PLAYER_BASE_SPEED }).abort();
    registry.add_component(player, DashComponent { dash: PLAYER_BASE_DASH, timeout: 1 }).abort();
    registry.add_component(player, CameraComponent {}).abort();

    for i in 0..10 {
        let e = registry.new_entity();
        registry.add_component(e, PositionComponent { x: (i * 8) as f32, y: (i * 8) as f32 }).unwrap();
        registry.add_component(e, HealthComponent { hp: i }).abort();
    }
}

#[no_mangle]
fn update() {
    let mut registry = unsafe { REGISTRY.lock().abort() };
    let mut waves = unsafe { WAVES.lock().abort() };

    move_system(&mut registry);
    draw_system(&mut registry);
    enemy_system(&mut registry);
    enemy_waves_system(&mut waves, &mut registry);
    registry.destroy_marked_entities();

    let mut topic: Topic<i32> = Topic::new();
    let mut sub_1 = Subscriber::new();
    let mut sub_2 = Subscriber::new();
    sub_1.follow(&mut topic);
    sub_2.follow(&mut topic);

    topic.send_message(123);
    topic.send_message(456);
    sub_1.pop_message().abort();
}


