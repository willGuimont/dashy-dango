use std::collections::LinkedList;

use crate::{Abort, Registry};
use crate::game::components::{CameraComponent, DashComponent, GamepadComponent, HealthComponent, MoveComponent, PositionComponent};
use crate::game::systems::{DrawSystem, MoveSystem, System};

const PLAYER_BASE_SPEED: i16 = 2;
const PLAYER_BASE_DASH: i16 = 5;

pub struct World {
    pub registry: Registry,
    pub systems: LinkedList<Box<dyn System>>,
}

impl World {
    pub fn new() -> Self { World { registry: Registry::new(), systems: LinkedList::new() } }

    pub fn create_player(&mut self, gamepad: *const u8) {
        let player = self.registry.new_entity();
        self.registry.add_component(player, PositionComponent { x: 0.0, y: 0.0 }).abort();
        self.registry.add_component(player, GamepadComponent { gamepad }).abort();
        self.registry.add_component(player, MoveComponent { speed: PLAYER_BASE_SPEED }).abort();
        self.registry.add_component(player, DashComponent { dash: PLAYER_BASE_DASH, timeout: 1 }).abort();
        self.registry.add_component(player, CameraComponent {}).abort();
    }

    pub fn create_systems(&mut self) {
        self.systems.push_back(Box::new(MoveSystem::new()));
        self.systems.push_back(Box::new(DrawSystem::new()));
    }

    pub fn create_entity(&mut self) {
        for i in 0..10 {
            let e = self.registry.new_entity();
            self.registry.add_component(e, PositionComponent { x: (i * 8) as f32, y: (i * 8) as f32 }).unwrap();
            self.registry.add_component(e, HealthComponent { hp: i }).abort();
        }
    }

    pub fn execute_systems(&mut self) {
        for system in self.systems.iter() {
            system.execute_system(&mut self.registry);
        }
    }
}
