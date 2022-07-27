use std::any::Any;

use ecs_macro::Component;

use wasm4::*;

use crate::ecs::{BaseComponent, Registry};
use crate::game::camera_component::CameraComponent;
use crate::utils::keyboard_utils;
use crate::math_utils::*;
use crate::game::components::position_component::PositionComponent;
use crate::game::dash_component::DashComponent;
use crate::game::direction_component::DirectionComponent;
use crate::game::draw_system::draw_entity;
use crate::game::gamepad_component::GamepadComponent;
use crate::game::health_component::HealthComponent;
use crate::game::move_component::MoveComponent;
use crate::game::move_system::process_player_movement;

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
const PLAYER_BASE_SPEED: i16 = 2;
const PLAYER_BASE_DASH: i16 = 5;

#[no_mangle]
fn update() {
    let mut registry = Registry::new();

    for i in 0..10 {
        let e = registry.new_entity();
        registry.add_component(e, PositionComponent { x: i as f32, y: i as f32 }).unwrap();
        if i % 2 == 0 {
            registry.add_component(e, HealthComponent { hp: i }).unwrap();
        }
    }

    trace("Listing entities with components");
    for (pos, health) in entities_with_components!(registry, PositionComponent, HealthComponent) {
        trace(format!("{:?}, {:?}", pos, health));
    }

    unsafe { *DRAW_COLORS = 2 }

    let gamepad = unsafe { *GAMEPAD1 };
    create_player(&mut registry, gamepad);
    process_player_movement(&registry);
    draw_entity(&registry);

}


fn create_player(registry: &mut Registry, gamepad:u8){
    let player = registry.new_entity();
    registry.add_component(player, PositionComponent { x: 0.0, y: 0.0 }).unwrap();
    registry.add_component(player, GamepadComponent { gamepad }).unwrap();
    registry.add_component(player, MoveComponent {speed: PLAYER_BASE_SPEED}).unwrap();
    registry.add_component(player, DashComponent {dash: PLAYER_BASE_DASH, timeout: 1 }).unwrap();
    registry.add_component(player, DirectionComponent {direction: Vec2::new(0.0, 0.0)}).unwrap();
    registry.add_component(player, CameraComponent {}).unwrap();
}