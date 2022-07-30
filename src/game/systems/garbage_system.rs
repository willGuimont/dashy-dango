use crate::game::systems::System;
use crate::Registry;

pub struct GarbageSystem {}

impl GarbageSystem {
    pub fn new() -> Self {
        GarbageSystem {}
    }
}

impl System for GarbageSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        registry.destroy_marked_entities();
    }
}
