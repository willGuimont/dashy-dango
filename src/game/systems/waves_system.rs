use crate::game::systems::System;
use crate::Registry;

pub struct WavesSystem {}

impl WavesSystem {
    pub fn new() -> Self {
        WavesSystem {}
    }
}

impl System for WavesSystem {
    fn execute_system(&self, registry: &mut Registry) {
        todo!()
    }
}
