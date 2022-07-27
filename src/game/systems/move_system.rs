use ecs_macro::Component;
use crate::*;
use crate::{DashComponent, entities_with_components, GamepadComponent, MoveComponent, PositionComponent, Registry, Vec2};
use crate::ecs::Entity;
use crate::keyboard_utils::gamepad_to_vec;
use crate::utils::is_dashing;


pub fn process_player_movement(registry:&mut Registry){
    for (e,(gamepad, dash, moveC ,pos)) in  entities_with_components!(registry, GamepadComponent, DashComponent, MoveComponent, PositionComponent) {
        let direction = gamepad_to_vec(gamepad.gamepad);
        if is_dashing(gamepad.gamepad) && dash.timeout == 0{
            //Process dash
        }else{
            move_player(direction, moveC, pos, registry, *e);
        }

    }
}

fn move_player(direction:Vec2, moveC :&MoveComponent, pos: &PositionComponent, registry: &mut Registry, e:Entity){
    let movement = direction * moveC.speed as f32;
    registry.add_component(e,PositionComponent {x:pos.x +movement.x, y:pos.y + movement.y}).unwrap();
}

