use std::collections::HashSet;
use std::collections::LinkedList;

use crate::{Abort, entities_with, entities_with_components, GameState, get_components_unwrap, has_all_components, Registry, set_game_state, Subscriber, Topic, Vec2};
use crate::assets::{DANGO_EYE_SPRITE, DANGO_SPRITE};
use crate::ecs::Entity;
use crate::game::components::{CameraComponent, ChildComponent, DangoEyeComponent, DashComponent, GameManagerComponent, GamepadComponent, HealthComponent, MoveComponent, PlayerComponent, PositionComponent, SizeComponent, SpriteComponent};
use crate::game::systems::{ChildSystem, DangoEyesSystem, DrawSystem, EnemyAttackSystem, EnemyMovementSystem, EnemyWavesSystem, HealthFlashSystem, HealthSystem, MoveSystem, NB_WAVES, ScoreSystem, System};
use crate::game::systems::ttl_system::TTLSystem;

const PLAYER_BASE_SPEED: i16 = 2;
const PLAYER_BASE_DASH: i16 = 60;
const PLAYER_BASE_HEALTH: i16 = 5;
const PLAYER_HIT_TIMEOUT: i16 = 100;
const BASE_SCORE: i32 = 100;
pub const WORLD_BOUNDARIES: f32 = 160.0;

pub struct World {
    pub registry: Registry,
    pub systems: LinkedList<Box<dyn System>>,
}

// TODO make world independent of our actual game, this logic should probably be in lib.rs, or some helper module
impl World {
    pub fn new() -> Self { World { registry: Registry::new(), systems: LinkedList::new() } }

    pub fn set(&mut self, gamepad: *const u8) {
        self.registry = Registry::new();
        self.systems = LinkedList::new();

        self.create_player(gamepad);
        self.create_game_manager();
        self.create_systems();
    }

    pub fn create_player(&mut self, gamepad: *const u8) {
        let player = self.registry.new_entity();
        self.registry.add_component(player, PlayerComponent).abort();
        self.registry.add_component(player, PositionComponent { pos: Vec2::new(0.0, 0.0) }).abort();
        self.registry.add_component(player, GamepadComponent { gamepad }).abort();
        self.registry.add_component(player, MoveComponent { speed: PLAYER_BASE_SPEED }).abort();
        self.registry.add_component(player, DashComponent { length: PLAYER_BASE_DASH, timeout: 10, duration: 0, direction: Vec2 { x: 0.0, y: 0.0 }, hit: HashSet::new() }).abort();
        self.registry.add_component(player, CameraComponent).abort();
        self.registry.add_component(player, SizeComponent { width: 8, height: 8 }).abort();
        self.registry.add_component(player, SpriteComponent { sprite: &DANGO_SPRITE, zindex: 2, is_visible: true }).abort();
        self.registry.add_component(player, HealthComponent { hp: PLAYER_BASE_HEALTH, timeout: 0, hit_delay: PLAYER_HIT_TIMEOUT }).abort();

        let eyes = self.registry.new_entity();
        self.registry.add_component(eyes, DangoEyeComponent).abort();
        self.registry.add_component(eyes, GamepadComponent { gamepad }).abort();
        self.registry.add_component(eyes, PositionComponent { pos: Vec2::new(0.0, 0.0) }).abort();
        self.registry.add_component(eyes, ChildComponent { parent: player, relative_pos: Vec2 { x: 3.0, y: 4.0 } }).abort();
        self.registry.add_component(eyes, SpriteComponent { sprite: &DANGO_EYE_SPRITE, zindex: 4, is_visible: true });
    }

    pub fn create_game_manager(&mut self) {
        let game_manager = self.registry.new_entity();
        self.registry.add_component(game_manager, GameManagerComponent { current_wave: 0, score: BASE_SCORE, player_hp: PLAYER_BASE_HEALTH }).abort();
    }

    pub fn create_systems(&mut self) {
        let mut score_event = Subscriber::new();
        let mut score_topic = Topic::new();
        score_event.follow(&mut score_topic);

        let mut health_event = Subscriber::new();
        let mut enemy_movement_system_health_topic = Topic::new();
        let mut move_system_health_topic = Topic::new();
        health_event.follow(&mut enemy_movement_system_health_topic);
        health_event.follow(&mut move_system_health_topic);

        self.systems.push_back(Box::new(MoveSystem { health_queue: move_system_health_topic }));
        self.systems.push_back(Box::new(ChildSystem));
        self.systems.push_back(Box::new(EnemyWavesSystem { current_wave: 0 }));
        self.systems.push_back(Box::new(EnemyMovementSystem { damage_topic: enemy_movement_system_health_topic }));
        self.systems.push_back(Box::new(EnemyAttackSystem));
        self.systems.push_back(Box::new(HealthSystem { event_queue: health_event, score_topic: score_topic }));
        self.systems.push_back(Box::new(HealthFlashSystem));
        self.systems.push_back(Box::new(TTLSystem));
        self.systems.push_back(Box::new(DangoEyesSystem));
        self.systems.push_back(Box::new(DrawSystem));
        self.systems.push_back(Box::new(ScoreSystem { score: BASE_SCORE, decrease_timer: 0, score_decrease_speed: 10, event_queue: score_event }));
    }

    pub fn execute_systems(&mut self) {
        match self.update_game_state() {
            GameState::Title => return (),
            GameState::Ongoing => for system in self.systems.iter_mut() {
                system.execute_system(&mut self.registry);
            },
            GameState::Win(score) => set_game_state(GameState::Win(score)),
            GameState::Loose(score, wave) => set_game_state(GameState::Loose(score, wave)),
        }
    }

    fn update_game_state(&mut self) -> GameState {
        let (_, (game_manager, )) = entities_with_components!(self.registry, GameManagerComponent).next().abort();

        if game_manager.current_wave >= NB_WAVES {
            return GameState::Win(game_manager.score);
        }
        if game_manager.player_hp <= 0 {
            return GameState::Loose(game_manager.score, game_manager.current_wave);
        }

        GameState::Ongoing
    }
}
