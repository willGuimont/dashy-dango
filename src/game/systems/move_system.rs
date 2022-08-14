use crate::*;
use crate::assets::{DANGO_DASH_SPRITE, DANGO_SPRITE};
use crate::ecs::Entity;
use crate::game::components::{DashComponent, EnemyComponent, GamepadComponent, HealthComponent, MoveComponent, PositionComponent, ScoreComponent, SizeComponent, SpriteComponent};
use crate::game::systems::system::System;
use crate::gamepad_utils::gamepad_to_vec;
use crate::utils::is_dashing;

pub struct MoveSystem {
    pub event_queue: Topic<i32>,
}

impl System for MoveSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry, GamepadComponent, DashComponent, MoveComponent, PositionComponent) {
            let (gamepad, dash, move_c, size, pos, sprite) = get_components_clone_unwrap!(registry, e, GamepadComponent, DashComponent, MoveComponent,SizeComponent, PositionComponent, SpriteComponent);
            let direction = gamepad_to_vec(gamepad.get_gamepad());

            if dash.duration > 0 {
                self.continue_dash(dash.direction, dash, size, pos, registry, e);
            } else if is_dashing(gamepad.get_gamepad()) && dash.timeout == 0 {
                self.process_dash(direction, dash, size, pos, sprite, registry, e);
            } else {
                self.move_player(direction, dash, move_c, pos, sprite, registry, e);
            }
        }
    }
}

impl MoveSystem {
    fn move_player(&mut self, direction: Vec2, mut dash: DashComponent, move_c: MoveComponent, mut pos: PositionComponent, mut sprite: SpriteComponent, registry: &mut Registry, e: Entity) {
        if dash.timeout > 0 {
            dash.timeout -= 1;
            if dash.timeout <= 0 {
                sprite.sprite = &DANGO_DASH_SPRITE;
            }
        }
        let movement = direction * move_c.speed as f32;
        pos.pos = pos.pos + movement;
        add_components!(registry, e, pos, dash, sprite);
    }

    fn process_dash(&mut self, dir: Vec2, mut dash: DashComponent, size: SizeComponent, mut pos: PositionComponent, mut sprite: SpriteComponent, registry: &mut Registry, e: Entity) {
        let direction = if dir.norm() == 0.0 { Vec2::new(1.0, 0.0) } else { dir };

        self.dash_damage(&mut dash, &size, &pos, registry);
        let segment_size = dash.length as f32 / size.width as f32;
        let segment = direction * segment_size;
        pos.pos = pos.pos + segment;
        dash.timeout += 25;
        dash.duration = segment_size as i16;
        dash.direction = direction;
        sprite.sprite = &DANGO_SPRITE;

        add_components!(registry, e, pos, dash, sprite);
    }

    fn continue_dash(&mut self, direction: Vec2, mut dash: DashComponent, size: SizeComponent, mut pos: PositionComponent, registry: &mut Registry, e: Entity) {
        self.dash_damage(&mut dash, &size, &pos, registry);
        let segment_size = dash.length as f32 / size.width as f32;
        let segment = direction * segment_size;
        pos.pos = pos.pos + segment;
        dash.duration -= 1;
        if dash.duration == 0 {
            self.kill_entity(&dash, registry);
            dash.hit.clear();
        }
        add_components!(registry, e, pos, dash);
    }

    fn dash_damage(&mut self, dash: &mut DashComponent, size: &SizeComponent, pos: &PositionComponent, registry: &mut Registry) {
        let player_hit = create_box(pos.pos, size.width as f32, size.height as f32);

        for e in entities_with!(registry, HealthComponent, SizeComponent, PositionComponent, EnemyComponent) {
            let (e_size, e_pos) = get_components_clone_unwrap!(registry,e, SizeComponent, PositionComponent);
            let entity_hit = create_box(e_pos.pos, e_size.width as f32, e_size.height as f32);

            if !dash.hit.contains(&e) && player_hit.rect_inter(&entity_hit) {
                dash.hit.insert(e);
            }
        }
    }

    fn kill_entity(&mut self, dash: &DashComponent, registry: &mut Registry) {
        for (i, &e) in dash.hit.iter().enumerate() {
            let (mut health, score) = get_components_clone_unwrap!(registry,e, HealthComponent, ScoreComponent);
            health.hp -= 1;
            if health.hp <= 0 {
                self.event_queue.send_message(score.score * (i + 1) as i32);
                registry.destroy_entity(e);
            } else {
                add_components!(registry, e, health);
            }
        }
    }
}
