use crate::*;
use crate::ecs::Entity;
use crate::game::components::{DashComponent, GamepadComponent, HealthComponent, MoveComponent, PositionComponent, SizeComponent};
use crate::game::systems::system::System;
use crate::gamepad_utils::gamepad_to_vec;
use crate::utils::is_dashing;

pub struct MoveSystem {}

impl System for MoveSystem {
    fn execute_system(&self, registry: &mut Registry) -> () {
        for e in entities_with!(registry, GamepadComponent, DashComponent, MoveComponent, SizeComponent,PositionComponent) {
            let (gamepad, dash, move_c, size, pos) = get_components_clone_unwrap!(registry,e,GamepadComponent,DashComponent, MoveComponent, SizeComponent, PositionComponent);
            let direction = gamepad_to_vec(gamepad.get_gamepad());

            if dash.is_dashing > 0 {
                continue_dash(dash.direction, dash, size, pos, registry, e);
            } else if is_dashing(gamepad.get_gamepad()) && dash.timeout == 0 {
                process_dash(direction, dash, size, pos, registry, e);
            } else {
                move_player(direction, dash, move_c, pos, registry, e);
            }
        }
    }
}

impl MoveSystem {
    pub fn new() -> Self { MoveSystem {} }
}

fn move_player(direction: Vec2, mut dash: DashComponent, move_c: MoveComponent, mut pos: PositionComponent, registry: &mut Registry, e: Entity) {
    //TODO make real timeout and remove dash from this function
    if dash.timeout > 0 {
        dash.timeout -= 1;
    }

    let movement = direction * move_c.speed as f32;
    pos.x += movement.x;
    pos.y += movement.y;
    add_components!(registry, e, pos, dash);
}

fn process_dash(direction: Vec2, mut dash: DashComponent, size: SizeComponent, mut pos: PositionComponent, registry: &mut Registry, e: Entity) {
    dash_damage(direction, &dash, &size, &pos, registry);
    let movement = direction * dash.dash as f32;
    pos.x += movement.x;
    pos.y += movement.y;
    dash.timeout += 10;
    dash.is_dashing = 10;
    dash.direction = direction;

    add_components!(registry, e, pos, dash);
}

fn continue_dash(direction: Vec2, mut dash: DashComponent, size: SizeComponent, mut pos: PositionComponent, registry: &mut Registry, e: Entity) {
    dash_damage(direction, &dash, &size, &pos, registry);
    let movement = direction * dash.dash as f32;
    pos.x += movement.x;
    pos.y += movement.y;
    dash.is_dashing -= 1;

    add_components!(registry, e, pos, dash);
}

fn dash_damage(direction: Vec2, dash: &DashComponent, size: &SizeComponent, pos: &PositionComponent, registry: &mut Registry) {
    let dash_hit = Quadrilateral::from_direction(direction, pos.x, pos.y, dash.dash as f32, size.height as f32);
    let dash_end_hit = Quadrilateral::from_direction(direction, pos.x + dash.dash as f32, pos.y, size.width as f32, size.height as f32);

    for e in entities_with!(registry, HealthComponent,SizeComponent, PositionComponent) {
        let (mut health, e_size, e_pos) = get_components_clone_unwrap!(registry,e,HealthComponent,SizeComponent, PositionComponent);
        let enemy_hit = Quadrilateral::from_direction(Vec2 { x: 1.0, y: 0.0 }, e_pos.x, e_pos.y, e_size.width as f32, e_size.height as f32);

        if dash_end_hit.rect_inter(&enemy_hit) {
            health.hp -= 5;
            if health.hp < 0 {
                registry.mark_to_destroy(e);
            }
            add_components!(registry, e, health);
        } else if dash_hit.rect_inter(&enemy_hit) {
            health.hp -= 1;
            if health.hp < 0 {
                registry.mark_to_destroy(e);
            }
            add_components!(registry, e, health);
        }
    }
}
