#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

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

static mut player_x:i32 = 76;
static mut player_y:i32 = 76;
const center:(i32,i32) = (76,76);

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 2 }
    let hello = camera_conversion(10,10);
    text("Hello from Rust!", hello.0, hello.1);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
    } if gamepad & BUTTON_UP != 0 {
        unsafe{player_y-=1;}
    } if gamepad & BUTTON_DOWN != 0 {
        unsafe{player_y+=1;}
    } if gamepad & BUTTON_RIGHT != 0 {
        unsafe{player_x+=1;}
    } if gamepad & BUTTON_LEFT != 0 {
        unsafe{player_x-=1;}
    }
    

    unsafe {let me = camera_conversion(player_x, player_y);
     blit(&SMILEY, me.0, me.1, 8, 8, BLIT_1BPP); }

    unsafe { *DRAW_COLORS = 4 }
    let other = camera_conversion(80,80);
    blit(&SMILEY, other.0, other.1, 8, 8, BLIT_1BPP); 

    let press = camera_conversion(16,90);
    text("Press X to dash", press.0, press.1);
}

fn camera_conversion (x:i32, y:i32)->(i32,i32){
    unsafe{(x-player_x+center.0, y-player_y+center.1)}
}
