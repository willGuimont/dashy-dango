#![feature(once_cell)]
#![feature(concat_idents)]
#![feature(iter_advance_by)]

use std::cell::OnceCell;

use wasm4::*;

use crate::abort::Abort;
use crate::ecs::{BaseComponent, Registry};
use crate::events::{Subscriber, Topic};
use crate::game::world::World;
use crate::math_utils::*;
use crate::utils::{gamepad_utils, int_to_string, is_dashing, is_pallete_changed};

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod math_utils;
mod ecs;
mod game;
mod utils;
mod events;
mod assets;
mod abort;

pub enum GameState {
    Title,
    Ongoing,
    Win(i32),
    Loose(i32, u8),
}

const PALETTES: [[u32; 4]; 2] = [
    [0xf99dec, 0xfc49e1, 0x88fce7, 0x34bca3],
    [0xFF73C3, 0xDB073D, 0x8EC7D2, 0x0D6986]
];

static mut WORLD: OnceCell<World> = OnceCell::new();
static mut GAME_STATE: GameState = GameState::Title;
static mut CURRENT_PALLETE: usize = 0;
static mut OLD_GAMEPAD: u8 = 0;

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = PALETTES[CURRENT_PALLETE];
        *DRAW_COLORS = 0x1320
    }
}

#[no_mangle]
fn update() {
    unsafe {
        match GAME_STATE {
            GameState::Title => begin_game(),
            GameState::Ongoing => execute_game(),
            GameState::Win(score) => win_game(score),
            GameState::Loose(score, wave) => loose_game(score, wave),
        }

        if is_pallete_changed(*GAMEPAD1 & (*GAMEPAD1 ^ OLD_GAMEPAD)) {
            CURRENT_PALLETE = (CURRENT_PALLETE + 1) % PALETTES.len();
            *PALETTE = PALETTES[CURRENT_PALLETE];
        }
        OLD_GAMEPAD = *GAMEPAD1;
    }
}

fn begin_game() {
    unsafe { *DRAW_COLORS = 0x4323 }

    text_centered("Welcome to", 5);
    text_centered("Dashy Dango!", 15);
    text_centered("Help the Dango", 30);
    text_centered("survive by fighting", 40);
    text_centered("waves of enemy", 50);

    text("Controls", 5, 70);
    text("-X to dash", 5, 80);
    text("-Z to switch", 5, 90);
    text("colour palette", 5, 98);

    text_centered(" Press x to start!", 130);

    unsafe {
        if is_dashing(*GAMEPAD1) {
            setup_world();
            set_game_state(GameState::Ongoing);
        }
    }
}

fn execute_game() {
    let world = unsafe { &mut WORLD.get_mut().abort() };
    world.execute_systems();
}

fn win_game(score: i32) {
    unsafe { *DRAW_COLORS = 0x4323 }
    text_centered("Congratulation!", 10);
    text_centered("You won the game", 20);
    text_centered("with", 30);
    text_centered(&int_to_string(score), 40);
    text_centered("points!", 50);

    text_centered("Press x", 100);
    text_centered("to play again!", 110);

    unsafe {
        if is_dashing(*GAMEPAD1) {
            reset_world();
            set_game_state(GameState::Ongoing);
        }
    }
}

fn loose_game(score: i32, wave: u8) {
    unsafe { *DRAW_COLORS = 0x4323 }
    text_centered("You lost the game", 10);
    text_centered("with", 20);
    text_centered(&int_to_string(score), 30);
    text_centered("points", 40);
    text_centered("on wave", 50);
    text_centered(&int_to_string(wave as i32), 60);

    text_centered("Press x", 100);
    text_centered("to try again!", 110);

    unsafe {
        if is_dashing(*GAMEPAD1) {
            reset_world();
            set_game_state(GameState::Ongoing);
        }
    }
}

pub fn set_game_state(game_state: GameState) {
    unsafe { GAME_STATE = game_state; }
}

fn setup_world() {
    let mut world = World::new();
    world.set(GAMEPAD1);
    unsafe { WORLD.set(world).abort(); }
}

fn reset_world() {
    let mut world = unsafe { &mut WORLD.get_mut().abort() };
    world.set(GAMEPAD1);
}

fn text_centered(message: &str, y: i32) {
    let x = ((20.0 - (message.len() as f32)) / 2.0 * 8.0) as i32;
    text(message, x, y);
}
