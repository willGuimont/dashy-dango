use crate::*;

// FIXME remove?
enum Direction {
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Still,
}

impl Direction {
    fn value(&self) -> (f32, f32) {
        match *self {
            Direction::Right => (1.0, 0.0),
            Direction::UpRight => (0.707, -0.707),
            Direction::Up => (0.0, -1.0),
            Direction::UpLeft => (-0.707, -0.707),
            Direction::Left => (-1.0, 0.0),
            Direction::DownLeft => (-0.707, 0.707),
            Direction::Down => (0.0, 1.0),
            Direction::DownRight => (0.707, 0.707),
            Direction::Still => (0.0, 0.0)
        }
    }
}

pub fn gamepad_to_vec(gamepad: u8) -> (f32, f32) {
    //TODO make constant for speed
    let speed = 2.0;
    let direction_value = get_direction(gamepad & 0b11110000).value();
    // TODO normalize
    (direction_value.0 * speed, direction_value.1 * speed)
}

fn get_direction(gamepad: u8) -> Direction {
    // FIXME use vec2 instead
    if (gamepad ^ (BUTTON_RIGHT | BUTTON_UP)) == 0 {
        Direction::UpRight
    } else if gamepad ^ (BUTTON_LEFT | BUTTON_UP) == 0 {
        Direction::UpLeft
    } else if gamepad ^ (BUTTON_RIGHT | BUTTON_DOWN) == 0 {
        Direction::DownRight
    } else if gamepad ^ (BUTTON_LEFT | BUTTON_DOWN) == 0 {
        Direction::DownLeft
    } else if gamepad & BUTTON_RIGHT != 0 {
        Direction::Right
    } else if gamepad & BUTTON_LEFT != 0 {
        Direction::Left
    } else if gamepad & BUTTON_UP != 0 {
        Direction::Up
    } else if gamepad & BUTTON_DOWN != 0 {
        Direction::Down
    } else {
        Direction::Still
    }
}