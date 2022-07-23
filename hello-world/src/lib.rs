#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
pub mod component;

use wasm4::*;
use component::*;

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

static mut player_x:f32 = 76.0;
static mut player_y:f32 = 76.0;
const center:(f32,f32) = (76.0,76.0);

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 2 }
    let hello = camera_conversion(10.0,10.0);
    text("Hello from Rust!", hello.0, hello.1);

    let gamepad = unsafe { *GAMEPAD1 };
    let direction = keyboardComponent::handle_controller(gamepad);
    unsafe{player_x+=direction.0;
           player_y+=direction.1;}

    unsafe {let me = camera_conversion(player_x, player_y);
     blit(&SMILEY, me.0, me.1, 8, 8, BLIT_1BPP); }

    unsafe { *DRAW_COLORS = 4 }
    let other = camera_conversion(80.0,80.0);
    blit(&SMILEY, other.0, other.1, 8, 8, BLIT_1BPP); 

    let press = camera_conversion(16.0,90.0);
    text("Press X to dash", press.0, press.1);
}

fn camera_conversion (x:f32, y:f32)->(i32,i32){
    unsafe{((x-player_x+center.0) as i32, (y-player_y+center.1) as i32)}
}
