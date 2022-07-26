extern crate proc_macro;

use wasm4::*;

use crate::ecs::{BaseComponent, Registry};
use crate::utils::keyboard_utils;
use crate::math_utils::*;

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

struct PositionComponent {
    x: i16,
    y: i16,
}

impl BaseComponent for PositionComponent {}

struct HealthComponent {
    hp: i16,
}

impl BaseComponent for HealthComponent {}

#[no_mangle]
fn update() {
    unsafe {
        let mut registry = Registry::new();
        let pos = PositionComponent { x: 1, y: 0 };
        let health = HealthComponent { hp: 10 };
        registry.add_component(1, pos);
        registry.add_component(1, health);
    }

    unsafe { *DRAW_COLORS = 2 }
    let hello = camera_conversion(10.0, 10.0);
    text("Hello from Rust!", hello.0, hello.1);

    let gamepad = unsafe { *GAMEPAD1 };
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

    test_inter();
}

fn camera_conversion(x: f32, y: f32) -> (i32, i32) {
    unsafe { ((x - PLAYER_X + CENTER.0) as i32, (y - PLAYER_Y + CENTER.1) as i32) }
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