use crate::*;
use crate::assets::{DANGO_DASH_SPRITE, DANGO_EYE_SPRITE, DANGO_SPRITE};
use crate::ecs::Entity;
use crate::game::components::{ChildComponent, DangoEyeComponent, DashComponent, DirectionComponent, EnemyComponent, GamepadComponent, HealthComponent, MoveComponent, PositionComponent, SizeComponent, SpriteComponent, TTLComponent};
use crate::game::systems::SoundEvent;
use crate::game::systems::system::System;
use crate::gamepad_utils::gamepad_to_vec;
use crate::utils::is_dashing;

pub struct MoveSystem {
    pub health_queue: Topic<(Entity, i32, i32)>,
    pub sound_queue: Topic<SoundEvent>,
}

impl System for MoveSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry, GamepadComponent, DashComponent, MoveComponent, PositionComponent) {
            let (gamepad, dash, move_c, size, pos, sprite) = get_components_clone_unwrap!(registry, e, GamepadComponent, DashComponent, MoveComponent,SizeComponent, PositionComponent, SpriteComponent);
            let direction = gamepad_to_vec(gamepad.get_gamepad());

            if dash.duration > 0 {
                self.continue_dash(registry, e, dash.direction, dash, size, pos);
            } else if is_dashing(gamepad.get_gamepad()) && dash.timeout == 0 {
                self.process_dash(registry, e, direction, dash, size, pos, sprite);
            } else {
                self.move_player(registry, e, direction, dash, move_c, pos, sprite);
            }
        }
    }
}

impl MoveSystem {
    fn move_player(&mut self, registry: &mut Registry, e: Entity, direction: Vec2, mut dash: DashComponent, move_c: MoveComponent, mut pos: PositionComponent, mut sprite: SpriteComponent) {
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

    fn process_dash(&mut self, registry: &mut Registry, e: Entity, dir: Vec2, mut dash: DashComponent, size: SizeComponent, mut pos: PositionComponent, mut sprite: SpriteComponent) {
        let direction = if dir.norm() == 0.0 { Vec2::new(1.0, 0.0) } else { dir };

        self.dash_damage(&mut dash, &size, &pos, registry);
        let segment_size = dash.length as f32 / size.width as f32;
        let segment = direction * segment_size;
        pos.pos = pos.pos + segment;
        dash.timeout += 25;
        dash.duration = segment_size as i16;
        dash.direction = direction;
        sprite.sprite = &DANGO_SPRITE;

        self.sound_queue.send_message(SoundEvent::Dash);

        add_components!(registry, e, pos, dash, sprite);
    }

    fn continue_dash(&mut self, registry: &mut Registry, e: Entity, direction: Vec2, mut dash: DashComponent, size: SizeComponent, mut pos: PositionComponent) {
        self.dash_damage(&mut dash, &size, &pos, registry);
        let segment_size = dash.length as f32 / size.width as f32;
        let segment = direction * segment_size;
        pos.pos = pos.pos + segment;
        dash.duration -= 1;
        if dash.duration == 0 {
            self.kill_entity(&dash);
            dash.hit.clear();
        }

        self.create_after_image(registry, pos.pos, direction);
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

    fn kill_entity(&mut self, dash: &DashComponent) {
        self.sound_queue.send_message(SoundEvent::Kill(dash.hit.len() as u32));
        let num_hit = dash.hit.len();
        for (i, &e) in dash.hit.iter().enumerate() {
            self.health_queue.send_message((e, 1, num_hit as i32));
        }
    }

    fn create_after_image(&mut self, registry: &mut Registry, after_image_pos: Vec2, direction: Vec2) {
        let after_image = registry.new_entity();
        registry.add_component(after_image, PositionComponent { pos: after_image_pos });
        registry.add_component(after_image, SpriteComponent { sprite: &DANGO_SPRITE, zindex: 0, is_visible: true });
        registry.add_component(after_image, TTLComponent { ttl: 5 });

        let eyes = registry.new_entity();
        registry.add_component(eyes, DirectionComponent { direction });
        registry.add_component(eyes, DangoEyeComponent);
        registry.add_component(eyes, PositionComponent { pos: Vec2::new(0.0, 0.0) }).abort();
        registry.add_component(eyes, ChildComponent { parent: after_image, relative_pos: Vec2 { x: 3.0, y: 4.0 } }).abort();
        registry.add_component(eyes, SpriteComponent { sprite: &DANGO_EYE_SPRITE, zindex: 1, is_visible: true });
    }
}
