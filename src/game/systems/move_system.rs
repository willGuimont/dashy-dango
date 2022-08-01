use std::hash::Hash;

use crate::*;
use crate::ecs::Entity;
use crate::game::components::{DashComponent, GamepadComponent, HealthComponent, MoveComponent, PositionComponent, SizeComponent};
use crate::game::systems::system::System;
use crate::gamepad_utils::gamepad_to_vec;
use crate::utils::is_dashing;

pub struct MoveSystem;

impl System for MoveSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry, GamepadComponent, DashComponent, MoveComponent, PositionComponent) {
            let (gamepad, dash, move_c, size, pos) = get_components_clone_unwrap!(registry,e,GamepadComponent,DashComponent, MoveComponent,SizeComponent, PositionComponent);
            let direction = gamepad_to_vec(gamepad.get_gamepad());

            if dash.duration > 0 {
                continue_dash(dash.direction, dash, size, pos, registry, e);
            } else if is_dashing(gamepad.get_gamepad()) && dash.timeout == 0 {
                process_dash(direction, dash, size, pos, registry, e);
            } else {
                move_player(direction, dash, move_c, pos, registry, e);
            }
        }
    }
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

fn process_dash(dir: Vec2, mut dash: DashComponent, size: SizeComponent, mut pos: PositionComponent, registry: &mut Registry, e: Entity) {
    let direction = if dir.norm() == 0.0 { Vec2::new(1.0, 0.0) } else { dir };

    dash_damage(&mut dash, &size, &pos, registry, e);
    let segment_size = dash.length as f32 / size.width as f32;
    let segment = direction * segment_size;
    pos.x += segment.x;
    pos.y += segment.y;
    dash.timeout += 10;
    dash.duration = segment_size as i16;
    dash.direction = direction;

    add_components!(registry, e, pos, dash);
}

fn continue_dash(direction: Vec2, mut dash: DashComponent, size: SizeComponent, mut pos: PositionComponent, registry: &mut Registry, e: Entity) {
    dash_damage(&mut dash, &size, &pos, registry, e);
    let segment_size = dash.length as f32 / size.width as f32;
    let segment = direction * segment_size;
    pos.x += segment.x;
    pos.y += segment.y;
    dash.duration -= 1;
    if dash.duration == 0 {
        kill_entity(&dash, registry, e);
        dash.hit.clear();
    }
    add_components!(registry, e, pos, dash);
}

fn dash_damage(dash: &mut DashComponent, size: &SizeComponent, pos: &PositionComponent, registry: &mut Registry, p_e: Entity) {
    let player_hit = create_hit_box(pos.x, pos.y, size.width as f32, size.height as f32);

    for e in entities_with!(registry, HealthComponent,SizeComponent, PositionComponent) {
        let (mut health, e_size, e_pos) = get_components_clone_unwrap!(registry,e,HealthComponent,SizeComponent, PositionComponent);
        let enemy_hit = create_hit_box(e_pos.x, e_pos.y, e_size.width as f32, e_size.height as f32);

        if e != p_e && player_hit.rect_inter(&enemy_hit) && !dash.hit.contains(&e) {
            dash.hit.insert(e);
        }
    }
}

fn kill_entity(dash: &DashComponent, registry: &mut Registry, p_e: Entity) {
    for &e in dash.hit.iter() {
        let (mut health, ) = get_components_clone_unwrap!(registry,e, HealthComponent);
        health.hp -= 1;
        if health.hp == 0 {
            registry.destroy_entity(e);
        } else {
            add_components!(registry, e, health);
        }
    }
}

fn create_hit_box(x: f32, y: f32, width: f32, height: f32) -> Quadrilateral {
    let p1 = Point::new(x, y);
    let p2 = Point::new(x, y + height);
    let p3 = Point::new(x + width, y + height);
    let p4 = Point::new(x + width, y);
    Quadrilateral::new([p1, p2, p3, p4])
}
