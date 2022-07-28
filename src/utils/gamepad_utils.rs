use crate::*;
use crate::math_utils::Vec2;

pub fn gamepad_to_vec(gamepad: u8) -> Vec2 {
    get_direction(gamepad)
}

pub fn is_dashing(gamepad: u8) -> bool {
    gamepad & BUTTON_1 != 0
}

fn get_direction(gamepad: u8) -> Vec2 {
    let mut direction = Vec2::new(0.0, 0.0);
    if gamepad & BUTTON_RIGHT != 0 {
        direction = direction + Vec2::new(1.0, 0.0);
    }
    if gamepad & BUTTON_LEFT != 0 {
        direction = direction + Vec2::new(-1.0, 0.0);
    }
    if gamepad & BUTTON_UP != 0 {
        direction = direction + Vec2::new(0.0, -1.0);
    }
    if gamepad & BUTTON_DOWN != 0 {
        direction = direction + Vec2::new(0.0, 1.0);
    }
    direction.normalized()
}
