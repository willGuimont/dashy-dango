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
const NB_HIGHSCORE_SAVED: usize = 5;

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
    text_centered("Dashy Dango!", 13);
    text_centered("Help the Dango", 30);
    text_centered("survive by fighting", 38);
    text_centered("waves of enemies.", 46);
    text_centered("Dash into enemies", 54);
    text_centered("to eliminate them", 62);

    text("Controls:", 5, 78);
    text("-X to dash", 5, 86);
    text("-Z to switch", 5, 94);
    text(" colour palette", 5, 102);

    text_centered(" Press x to start!", 140);

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
    let highscore = read_saved_highscore();

    unsafe { *DRAW_COLORS = 0x4323 }
    text_centered("Congratulation!", 10);
    text_centered("You won the game", 18);
    text_centered("with", 26);
    text_centered(&int_to_string(score), 34);
    text_centered("points!", 42);

    text_centered("Highscores", 60);
    for i in 0..NB_HIGHSCORE_SAVED {
        text_centered(&int_to_string(highscore[i]), (68 + (i * 8)) as i32)
    }

    text_centered("Press x", 130);
    text_centered("to play again!", 138);

    unsafe {
        if is_dashing(*GAMEPAD1) {
            reset_world();
            set_game_state(GameState::Ongoing);
        }
    }
}

fn loose_game(score: i32, wave: u8) {
    let highscore = read_saved_highscore();

    unsafe { *DRAW_COLORS = 0x4323 }
    text_centered("You lost the game", 10);
    text_centered("with", 18);
    text_centered(&int_to_string(score), 26);
    text_centered("points", 34);
    text_centered("on wave", 42);
    text_centered(&int_to_string(wave as i32), 50);

    text_centered("Highscores", 70);
    for i in 0..NB_HIGHSCORE_SAVED {
        text_centered(&int_to_string(highscore[i]), (78 + (i * 8)) as i32)
    }

    text_centered("Press x", 130);
    text_centered("to try again!", 138);

    unsafe {
        if is_dashing(*GAMEPAD1) {
            reset_world();
            set_game_state(GameState::Ongoing);
        }
    }
}

pub fn set_game_state(game_state: GameState) {
    match game_state {
        GameState::Title => (),
        GameState::Ongoing => (),
        GameState::Win(score) => write_new_highscore(score),
        GameState::Loose(score, _) => write_new_highscore(score),
    }
    unsafe { GAME_STATE = game_state; }
}

fn setup_world() {
    let mut world = World::new();
    world.set(GAMEPAD1);
    unsafe { WORLD.set(world).abort(); }
}

fn reset_world() {
    let world = unsafe { &mut WORLD.get_mut().abort() };
    world.set(GAMEPAD1);
}

fn text_centered(message: &str, y: i32) {
    let x = ((20.0 - (message.len() as f32)) / 2.0 * 8.0) as i32;
    text(message, x, y);
}

fn read_saved_highscore() -> [i32; NB_HIGHSCORE_SAVED] {
    let mut game_data = [0; NB_HIGHSCORE_SAVED];
    unsafe {
        let mut buffer = [0; NB_HIGHSCORE_SAVED * 4];
        diskr(buffer.as_mut_ptr(), buffer.len() as u32);

        let mut int_buffer = [[0; 4]; NB_HIGHSCORE_SAVED];
        for i in 0..NB_HIGHSCORE_SAVED * 4 {
            int_buffer[i / 4][i % 4] = buffer[i];
        }
        for i in 0..int_buffer.len() {
            game_data[i] = i32::from_le_bytes(int_buffer[i]);
        }
    };

    game_data
}

fn write_new_highscore(score: i32) {
    let game_data = read_saved_highscore();
    let new_game_data = compare_highscore(score, game_data);
    let mut buffer = [[0; 4]; NB_HIGHSCORE_SAVED];
    for i in 0..NB_HIGHSCORE_SAVED {
        buffer[i] = new_game_data[i].to_le_bytes();
    }
    let mut game_data_bytes = [0; NB_HIGHSCORE_SAVED * 4];
    for i in 0..NB_HIGHSCORE_SAVED * 4 {
        game_data_bytes[i] = buffer[i / 4][i % 4];
    }

    unsafe {
        diskw(game_data_bytes.as_ptr(), core::mem::size_of::<[i32; NB_HIGHSCORE_SAVED]>() as u32);
    }
}

fn compare_highscore(score: i32, game_data: [i32; NB_HIGHSCORE_SAVED as usize]) -> [i32; NB_HIGHSCORE_SAVED] {
    for i in 0..NB_HIGHSCORE_SAVED {
        if score > game_data[i] {
            let mut new_data = [0; NB_HIGHSCORE_SAVED];
            for j in 0..NB_HIGHSCORE_SAVED {
                if i > j {
                    new_data[j] = game_data[j];
                } else if i == j {
                    new_data[j] = score;
                } else {
                    new_data[j] = game_data[j - 1];
                }
            }

            return new_data;
        }
    }

    return game_data;
}
