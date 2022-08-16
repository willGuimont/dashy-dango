use crate::{entities_with, entities_with_components, get_components_clone_unwrap, get_components_unwrap, has_all_components, Registry, trace};
use crate::abort::Abort;
use crate::ecs::Entity;
use crate::game::components::{HealthComponent, PlayerComponent, SpriteComponent};
use crate::game::systems::System;

pub struct HealthFlashSystem;

impl System for HealthFlashSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        let (&e, _) = entities_with_components!(registry, PlayerComponent, HealthComponent, SpriteComponent).next().abort();
        let (health, mut sprite) = get_components_clone_unwrap!(registry, e, HealthComponent, SpriteComponent);
        if health.timeout > 0 && health.timeout % 3 == 0 {
            sprite.is_visible = !sprite.is_visible;
            registry.add_component(e, sprite);
        } else if health.timeout == 0 && !sprite.is_visible {
            sprite.is_visible = !sprite.is_visible;
            registry.add_component(e, sprite);
        }
    }
}
