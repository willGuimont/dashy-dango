use crate::{entities_with, get_components_clone_unwrap, has_all_components, Registry};
use crate::abort::Abort;
use crate::ecs::Entity;
use crate::game::components::{HealthComponent, SpriteComponent};
use crate::game::systems::System;

pub struct HealthFlashSystem;

impl System for HealthFlashSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry, HealthComponent, SpriteComponent) {
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
}
