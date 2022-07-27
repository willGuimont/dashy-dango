use std::any::Any;

use ecs_macro::Component;

use wasm4::*;

use crate::abort::Abort;
use crate::ecs::{BaseComponent, Registry, Entity};
use crate::game::camera_component::CameraComponent;
use crate::events::{Subscriber, Topic};
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



#[no_mangle]
fn start() {
    let mut registry = Registry::new();
    create_player(&mut registry);
}

#[no_mangle]
fn update() {
    for i in 0..10 {
        let e = registry.new_entity();
        registry.add_component(e, PositionComponent { x: i as f32, y: i as f32 }).unwrap();
        registry.add_component(e, HealthComponent { hp: i }).abort();
    }
        for (_, (pos, health)) in entities_with_components!(registry, PositionComponent, HealthComponent) {
        }

        // mut
        for e in entities_with!(registry, PositionComponent, HealthComponent) {
            let (mut pos, mut health) = get_components_clone_unwrap!(registry, e, PositionComponent, HealthComponent);
            pos.x += 1 as f32;
            health.hp = 100;
            add_components!(&mut registry, e, pos, health);
        }

        for (_, (pos, health)) in entities_with_components!(registry, PositionComponent, HealthComponent) {
        }

        let mut topic: Topic<i32> = Topic::new();
        let mut sub_1 = Subscriber::new();
        let mut sub_2 = Subscriber::new();
        sub_1.follow(&mut topic);
        sub_2.follow(&mut topic);

        topic.send_message(123);
        topic.send_message(456);
        sub_1.pop_message().abort();

        unsafe { *DRAW_COLORS = 2 }
    
        process_player_movement(&mut registry);
        draw_entity(&registry);
    }



fn create_player(registry: &mut Registry){
    let gamepad = unsafe { *GAMEPAD1 };
    let player = registry.new_entity();
    registry.add_component(player, PositionComponent { x: 0.0, y: 0.0 }).unwrap();
    registry.add_component(player, GamepadComponent { gamepad }).unwrap();
    registry.add_component(player, MoveComponent {speed: PLAYER_BASE_SPEED}).unwrap();
    registry.add_component(player, DashComponent {dash: PLAYER_BASE_DASH, timeout: 1 }).unwrap();
    registry.add_component(player, DirectionComponent {direction: Vec2::new(0.0, 0.0)}).unwrap();
    registry.add_component(player, CameraComponent {}).unwrap();
}