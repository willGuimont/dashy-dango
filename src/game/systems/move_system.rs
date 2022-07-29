use crate::*;
use crate::ecs::Entity;
use crate::game::components::{DashComponent, GamepadComponent, MoveComponent, PositionComponent};
use crate::game::systems::system::System;
use crate::gamepad_utils::gamepad_to_vec;
use crate::utils::is_dashing;

pub struct MoveSystem {}

impl System for MoveSystem {
    fn execute_system(&self, registry: &mut Registry) -> () {
        for e in entities_with!(registry, GamepadComponent, DashComponent, MoveComponent, PositionComponent) {
            let (gamepad, dash, move_c, pos) = get_components_clone_unwrap!(registry,e,GamepadComponent,DashComponent, MoveComponent,PositionComponent);

            let direction = gamepad_to_vec(gamepad.get_gamepad());
            if is_dashing(gamepad.get_gamepad()) && dash.timeout == 0 {
                //Process dash
            } else {
                move_player(direction, move_c, pos, registry, e);
            }
        }
    }
}

impl MoveSystem {
    pub fn new() -> Self { MoveSystem {} }
}

fn move_player(direction: Vec2, move_c: MoveComponent, mut pos: PositionComponent, registry: &mut Registry, e: Entity) {
    let movement = direction * move_c.speed as f32;
    pos.x += movement.x;
    pos.y += movement.y;
    add_components!(registry, e, pos);
}
