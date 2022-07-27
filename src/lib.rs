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
use crate::game::gamepad_component::GamepadComponent;
use crate::game::health_component::HealthComponent;
use crate::game::move_component::MoveComponent;

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
    let hello = camera_conversion(10.0, 10.0);
    text("Hello from Rust!", hello.0, hello.1);

    let gamepad = unsafe { *GAMEPAD1 };
    create_player(&mut registry, gamepad);

    let direction = keyboard_utils::gamepad_to_vec(gamepad);
    unsafe {
        PLAYER_X += direction.x;
        PLAYER_Y += direction.y;
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

    //test_inter();
}


fn test_inter(){
    let quad = Quadrilateral::new([Point::new(0.0,0.0),Point::new(1.0,0.0),Point::new(1.0,1.0),Point::new(0.0,1.0)]);
    let quad2 = Quadrilateral::new([Point::new(-1.0,-1.0),Point::new(0.5,-1.0),Point::new(0.5,1.5),Point::new(-1.0,1.5)]);
    if quad.rect_inter(quad2){
         trace("true");
    }
    else {
         trace("false");
    }

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