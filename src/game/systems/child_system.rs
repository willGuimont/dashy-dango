use crate::{Abort, entities_with, get_components_clone_unwrap, has_all_components, Registry};
use crate::ecs::Entity;
use crate::game::components::{ChildComponent, PositionComponent};
use crate::game::systems::System;

pub struct ChildSystem;

impl System for ChildSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry, ChildComponent) {
            let child = registry.get_component::<ChildComponent>(e).abort().clone();
            if !registry.is_alive(child.parent) {
                registry.destroy_entity(e);
            }

            if let Some(mut pos) = registry.get_component::<PositionComponent>(e).cloned() {
                if let Some(parent_pos) = registry.get_component::<PositionComponent>(child.parent) {
                    pos.pos = parent_pos.pos + child.relative_pos;
                    registry.add_component(e, pos);
                }
            }
        }
    }
}
