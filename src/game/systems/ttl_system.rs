use crate::*;
use crate::{entities_with, get_components_clone_unwrap, Registry};
use crate::ecs::Entity;
use crate::game::components::ttl_component::TTLComponent;
use crate::game::systems::System;

pub struct TTLSystem;

impl System for TTLSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry, TTLComponent) {
            let (mut ttl, ) = get_components_clone_unwrap!(registry,e,TTLComponent);

            if ttl.ttl > 0 {
                ttl.ttl -= 1;
                registry.add_component(e, ttl);
            } else {
                registry.destroy_entity(e);
            }
        }
    }
}
