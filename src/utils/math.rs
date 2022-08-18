use std::f32::consts::{PI, TAU};

pub fn wrap_to_pi(x: f32) -> f32 {
    let x = (x + PI) % TAU;
    if x >= 0.0 {
        x - PI
    } else {
        x + PI
    }
}

pub fn sin(x: f32) -> f32 {
    let x = wrap_to_pi(x);
    let x3 = x * x * x;
    x - x3 / 6.0 + x3 * x * x / 120.0 - x3 * x3 * x / 5040.0
}

pub fn cos(x: f32) -> f32 {
    let x = wrap_to_pi(x);
    let x2 = x * x;
    let x4 = x2 * x2;
    1.0 - x2 / 2.0 + x4 / 24.0 - x2 * x4 / 720.0 + x4 * x4 / 40320.0
}
