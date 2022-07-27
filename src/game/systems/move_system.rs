use ecs_macro::Component;
use crate::*;
use crate::{DashComponent, entities_with_components, GamepadComponent, MoveComponent, PositionComponent, Registry, Vec2};
use crate::keyboard_utils::gamepad_to_vec;
use crate::utils::is_dashing;


pub fn process_player_movement(registry:&Registry){
    for (gamepad, dash, moveC ,pos) in  entities_with_components!(registry, GamepadComponent, DashComponent, MoveComponent, PositionComponent) {
        let direction = gamepad_to_vec(gamepad.gamepad);
        if is_dashing(gamepad.gamepad) && dash.timeout == 0{
            //Process dash
        }else{
            //move_player(direction, moveC, pos);
        }

    }
}

fn move_player(direction:Vec2, moveC :&MoveComponent, pos: &mut PositionComponent){
    let movement = direction * moveC.speed as f32;
    pos.x += movement.x;
    pos.y += movement.y;
}

