#![feature(once_cell)]
#![feature(concat_idents)]

use std::cell::OnceCell;

use wasm4::*;

use crate::abort::Abort;
use crate::ecs::{BaseComponent, Registry};
use crate::events::{Subscriber, Topic};
use crate::game::world::World;
use crate::math_utils::*;
use crate::utils::gamepad_utils;

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

static mut WORLD: OnceCell<World> = OnceCell::new();

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [
            0xf99dec,
            0xfc49e1,
            0x88fce7,
            0x34bca3];
        *DRAW_COLORS = 0x4320
    }
    let mut world = World::new();

    world.create_player(GAMEPAD1);
    world.create_systems();
    unsafe { WORLD.set(world).abort() };
}

#[no_mangle]
fn update() {
    let world = unsafe { &mut WORLD.get_mut().abort() };
    world.execute_systems();

    let mut topic: Topic<i32> = Topic::new();
    let mut sub_1 = Subscriber::new();
    let mut sub_2 = Subscriber::new();
    sub_1.follow(&mut topic);
    sub_2.follow(&mut topic);

    topic.send_message(123);
    topic.send_message(456);
    sub_1.pop_message().abort();
}


