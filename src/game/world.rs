use std::collections::HashSet;
use std::collections::LinkedList;

use crate::{Abort, Registry, Subscriber, Topic, Vec2};
use crate::assets::{DANGO_EYE_SPRITE, DANGO_OUTLINE_SPRITE, DANGO_SPRITE};
use crate::game::components::{CameraComponent, ChildComponent, DangoEyeComponent, DashComponent, GamepadComponent, HealthComponent, MoveComponent, PlayerComponent, PositionComponent, SizeComponent, SpriteComponent};
use crate::game::systems::{ChildSystem, DangoEyesSystem, DrawSystem, EnemyAttackSystem, EnemyMovementSystem, EnemyWavesSystem, MoveSystem, ScoreSystem, System};
use crate::game::systems::ttl_system::TTLSystem;

const PLAYER_BASE_SPEED: i16 = 2;
const PLAYER_BASE_DASH: i16 = 60;
const PLAYER_BASE_HEALTH: i16 = 5;
const PLAYER_HIT_TIMEOUT: i16 = 1000;

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
        self.registry.add_component(player, SpriteComponent { sprite: &DANGO_SPRITE, zindex: 0 }).abort();
        self.registry.add_component(player, HealthComponent { hp: PLAYER_BASE_HEALTH, timeout: 0, hit_delay: PLAYER_HIT_TIMEOUT }).abort();

        let eyes = self.registry.new_entity();
        self.registry.add_component(eyes, DangoEyeComponent).abort();
        self.registry.add_component(eyes, GamepadComponent { gamepad }).abort();
        self.registry.add_component(eyes, PositionComponent { pos: Vec2::new(0.0, 0.0) }).abort();
        self.registry.add_component(eyes, ChildComponent { parent: player, relative_pos: Vec2 { x: 3.0, y: 4.0 } }).abort();
        self.registry.add_component(eyes, SpriteComponent { sprite: &DANGO_EYE_SPRITE, zindex: 2 });

        let outline = self.registry.new_entity();
        self.registry.add_component(outline, PositionComponent { pos: Vec2::new(0.0, 0.0) }).abort();
        self.registry.add_component(outline, ChildComponent { parent: player, relative_pos: Vec2 { x: 0.0, y: 0.0 } }).abort();
        self.registry.add_component(outline, SpriteComponent { sprite: &DANGO_OUTLINE_SPRITE, zindex: 0 });
    }

    pub fn create_systems(&mut self) {
        let mut score_event = Subscriber::new();
        let mut score_topic = Topic::new();
        score_event.follow(&mut score_topic);

        self.systems.push_back(Box::new(MoveSystem { event_queue: score_topic }));
        self.systems.push_back(Box::new(ChildSystem));
        self.systems.push_back(Box::new(EnemyWavesSystem { current_wave: 0 }));
        self.systems.push_back(Box::new(EnemyMovementSystem));
        self.systems.push_back(Box::new(EnemyAttackSystem));
        self.systems.push_back(Box::new(TTLSystem));
        self.systems.push_back(Box::new(DangoEyesSystem));
        self.systems.push_back(Box::new(DrawSystem));
        self.systems.push_back(Box::new(ScoreSystem { score: 100, decrease_timer: 0, score_decrease_speed: 10, event_queue: score_event }));
    }

    pub fn execute_systems(&mut self) {
        for system in self.systems.iter_mut() {
            system.execute_system(&mut self.registry);
        }
    }
}
