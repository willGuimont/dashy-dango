use ecs_macro::Component;

use crate::*;
use crate::{DashComponent, entities_with_components, GamepadComponent, MoveComponent, PositionComponent, Registry, Vec2};
use crate::ecs::Entity;
use crate::gamepad_utils::gamepad_to_vec;
use crate::utils::is_dashing;

pub fn process_player_movement(registry: &mut Registry) {
    for (e) in entities_with!(registry, GamepadComponent, DashComponent, MoveComponent, PositionComponent) {
        let (gamepad, dash, move_c, mut pos) = get_components_clone_unwrap!(registry,e,GamepadComponent,DashComponent, MoveComponent,PositionComponent);

        let direction = gamepad_to_vec(gamepad.get_gamepad());
        if is_dashing(gamepad.get_gamepad()) && dash.timeout == 0 {
            //Process dash
        } else {
            move_player(direction, move_c, pos, registry, e);
        }
    }
}

fn move_player(direction: Vec2, move_c: MoveComponent, mut pos: PositionComponent, registry: &mut Registry, e: Entity) {
    let movement = direction * move_c.speed as f32;
    pos.x += movement.x;
    pos.y += movement.y;
    add_components!(registry, e, pos);
}
