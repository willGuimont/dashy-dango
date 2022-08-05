use crate::{Abort, create_box, entities_with, entities_with_components, get_components_clone_unwrap, get_components_unwrap, has_all_components, Registry, trace, Vec2};
use crate::ecs::Entity;
use crate::game::components::{EnemyComponent, HealthComponent, PlayerComponent, PositionComponent, SizeComponent};
use crate::game::systems::System;

pub struct EnemySystem;

impl System for EnemySystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let (_, (_, player_pos)) = entities_with_components!(registry, PlayerComponent, PositionComponent).next().abort();
        let player_pos = player_pos.pos;
        for e in entities_with!(registry, EnemyComponent) {
            let (enemy, enemy_pos, enemy_size) = get_components_clone_unwrap!(registry, e, EnemyComponent, PositionComponent, SizeComponent);
            let enemy_pos = enemy_pos.pos;
            let direction_to_player = (player_pos - enemy_pos).normalized();

            let enemy_pos = enemy_pos + direction_to_player * enemy.speed;

            collide_player(enemy_pos, &enemy_size, registry);
            registry.add_component(e, PositionComponent { pos: enemy_pos });
        }
    }
}

fn collide_player(e_pos: Vec2, e_size: &SizeComponent, registry: &mut Registry) {
    for e in entities_with!(registry, PlayerComponent) {
        let (mut health, p_pos, p_size) = get_components_clone_unwrap!(registry,e,HealthComponent,PositionComponent,SizeComponent);
        let player_hit = create_box(p_pos.pos, p_size.width as f32, p_size.height as f32);
        let enemy_hit = create_box(e_pos, e_size.width as f32, e_size.height as f32);
        if health.timeout > 0 {
            health.timeout -= 1;
            registry.add_component(e, health);
        } else if health.timeout <= 0 && enemy_hit.rect_inter(&player_hit) {
            health.hp -= 1;
            health.timeout += health.hit_delay;
            registry.add_component(e, health);
        }
    }
}
