use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

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

static mut WORLD: Lazy<Arc<Mutex<World>>> = Lazy::new(|| Arc::new(Mutex::new(World::new())));

#[no_mangle]
fn start() {
    unsafe { *DRAW_COLORS = 2 }

    let world = &mut unsafe { WORLD.lock().abort() };

    world.create_player(GAMEPAD1);
    world.create_systems();
}

#[no_mangle]
fn update() {
    let mut world = unsafe { WORLD.lock().abort() };
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


