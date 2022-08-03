use std::collections::HashSet;
use std::collections::LinkedList;

use crate::{Abort, REFRESH_RATE, Registry, Vec2};
use crate::assets::DANGO_SPRITE;
use crate::game::components::{CameraComponent, ChildComponent, DashComponent, GamepadComponent, MoveComponent, PlayerComponent, PositionComponent, SizeComponent, SpriteComponent};
use crate::game::systems::{ChildSystem, DrawSystem, EnemySystem, EnemyWavesSystem, MoveSystem, System};

const PLAYER_BASE_SPEED: i16 = 2;
const PLAYER_BASE_DASH: i16 = 60;

pub struct World {
    pub registry: Registry,
    pub systems: LinkedList<Box<dyn System>>,
}

// TODO make world independent of our actual game, this logic should probably be in lib.rs, or some helper module
impl World {
    pub fn new() -> Self { World { registry: Registry::new(), systems: LinkedList::new() } }

    pub fn create_player(&mut self, gamepad: *const u8) {
        let player = self.registry.new_entity();
        self.registry.add_component(player, PlayerComponent).abort();
        self.registry.add_component(player, PositionComponent { pos: Vec2::new(0.0, 0.0) }).abort();
        self.registry.add_component(player, GamepadComponent { gamepad }).abort();
        self.registry.add_component(player, MoveComponent { speed: PLAYER_BASE_SPEED }).abort();
        self.registry.add_component(player, DashComponent { length: PLAYER_BASE_DASH, timeout: 0, duration: 0, direction: Vec2 { x: 0.0, y: 0.0 }, hit: HashSet::new() }).abort();
        self.registry.add_component(player, CameraComponent).abort();
        self.registry.add_component(player, SizeComponent { width: 8, height: 8 }).abort();
        self.registry.add_component(player, SpriteComponent { sprite: &DANGO_SPRITE });

        let child = self.registry.new_entity();
        self.registry.add_component(child, PositionComponent { pos: Vec2::new(0.0, 0.0) }).abort();
        self.registry.add_component(child, ChildComponent { parent: player, relative_pos: Vec2 { x: 10.0, y: 0.0 } }).abort();
        self.registry.add_component(child, SpriteComponent { sprite: &DANGO_SPRITE });
    }

    pub fn create_systems(&mut self) {
        // TODO might consider adding a macro to remove all this boilerplate
        self.systems.push_back(Box::new(MoveSystem));
        self.systems.push_back(Box::new(ChildSystem));
        self.systems.push_back(Box::new(DrawSystem));
        self.systems.push_back(Box::new(EnemySystem));
        self.systems.push_back(Box::new(EnemyWavesSystem::new(10 * (REFRESH_RATE as i32))));
    }

    pub fn execute_systems(&mut self) {
        for system in self.systems.iter_mut() {
            system.execute_system(&mut self.registry);
        }
    }
}
