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
use crate::utils::{gamepad_utils, int_to_string, is_dashing};

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

static mut WORLD: OnceCell<World> = OnceCell::new();
static mut GAME_STATE: GameState = GameState::Title;


#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [
            0xf99dec,
            0xfc49e1,
            0x88fce7,
            0x34bca3];
        *DRAW_COLORS = 0x1320
    }
    let mut world = World::new();

    world.create_player(GAMEPAD1);
    world.create_game_manager();
    world.create_systems();
    unsafe { WORLD.set(world).abort() };
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
    }
}

fn begin_game() {
    text("Welcome to Dashy Dango!", 10, 10);
    text("Help the Dango survive by fightings waves of enemy", 10, 30);
    text("Controls", 10, 50);
    text("-X to dash", 10, 70);
    text("-Z to switch colour palette", 10, 90);
    text(" Press x to start!", 10, 110);

    unsafe {
        if is_dashing(*GAMEPAD1) {
            set_game_state(GameState::Ongoing);
        }
    }
}

fn execute_game() {
    let world = unsafe { &mut WORLD.get_mut().abort() };
    world.execute_systems();
}

fn win_game(score: i32) {
    let string_score = "You won the game with ".to_owned() + &int_to_string(score);
    let string_score = string_score + " points!";
    text("Congratulation!", 10, 10);
    text(string_score, 10, 30);
}

fn loose_game(score: i32, wave: u8) {
    let string_score = "You lost the game with ".to_owned() + &int_to_string(score);
    let string_score = string_score + " points!";
    let wave = "on wave ".to_owned() + &int_to_string(wave as i32);
    text(string_score, 10, 30);
    text(wave, 10, 50);
}

pub fn set_game_state(game_state: GameState) {
    unsafe { GAME_STATE = game_state; }
}
