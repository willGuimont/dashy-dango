use std::concat_idents;

use crate::{BLIT_1BPP, BLIT_2BPP, BLIT_FLIP_X, BLIT_FLIP_Y, BLIT_ROTATE};

const DANGO_WIDTH: u32 = 8;
const DANGO_HEIGHT: u32 = 8;
const DANGO_FLAGS: u32 = BLIT_2BPP;
const DANGO: [u8; 16] = [0x00, 0x00, 0x05, 0x50, 0x1a, 0xa4, 0x6a, 0xa9, 0x6a, 0xa9, 0x6a, 0xa9, 0x6a, 0xa9, 0x15, 0x54];
const DANGO_DRAW: u16 = 0x0341;

const DANGO_DASH_WIDTH: u32 = 8;
const DANGO_DASH_HEIGHT: u32 = 8;
const DANGO_DASH_FLAGS: u32 = BLIT_2BPP;
const DANGO_DASH: [u8; 16] = DANGO;
const DANGO_DASH_DRAW: u16 = 0x0321;

const GRASS_WIDTH: u32 = 8;
const GRASS_HEIGHT: u32 = 8;
const GRASS_FLAGS: u32 = BLIT_1BPP;
const GRASS: [u8; 8] = [0x00, 0x42, 0x44, 0x29, 0x12, 0x14, 0x14, 0x08];
const GRASS_DRAW: u16 = 0x0030;

const DANGO_EYE_WIDTH: u32 = 3;
const DANGO_EYE_HEIGHT: u32 = 2;
const DANGO_EYE_FLAGS: u32 = BLIT_1BPP;
const DANGO_EYE: [u8; 1] = [0b10110100];
const DANGO_EYE_DRAW: u16 = 0x0020;

const BULLET_WIDTH: u32 = 2;
const BULLET_HEIGHT: u32 = 2;
const BULLET_FLAGS: u32 = BLIT_1BPP;
const BULLET: [u8; 1] = [0b11110000];
const BULLET_DRAW: u16 = 0x0040;

const SPITWORM_WIDTH: u32 = 8;
const SPITWORM_HEIGHT: u32 = 8;
const SPITWORM_FLAGS: u32 = BLIT_2BPP;
const SPITWORM: [u8; 16] = [0x14, 0x14, 0x69, 0x69, 0x15, 0x54, 0x07, 0xd0, 0x07, 0xd0, 0x05, 0x50, 0x01, 0x00, 0x01, 0x55];
const SPITWORM_DRAW: u16 = 0x1320;

const FLY_WIDTH: u32 = 8;
const FLY_HEIGHT: u32 = 8;
const FLY_FLAGS: u32 = BLIT_2BPP;
const FLY: [u8; 16] = [0x10, 0x04, 0x64, 0x19, 0x64, 0x19, 0x15, 0x54, 0x06, 0x90, 0x1a, 0xa4, 0x1a, 0xa4, 0x05, 0x50];
const FLY_DRAW: u16 = 0x1320;

const ROBOT_WIDTH: u32 = 8;
const ROBOT_HEIGHT: u32 = 8;
const ROBOT_FLAGS: u32 = BLIT_2BPP;
const ROBOT: [u8; 16] = [0x00, 0x00, 0x15, 0x54, 0x04, 0x10, 0x84, 0x12, 0x85, 0x52, 0x85, 0x52, 0x85, 0x52, 0x80, 0x02];
const ROBOT_DRAW: u16 = 0x2132;

const TOMBSTONE_WIDTH: u32 = 8;
const TOMBSTONE_HEIGHT: u32 = 8;
const TOMBSTONE_FLAGS: u32 = BLIT_2BPP;
const TOMBSTONE: [u8; 16] = [0x05, 0x00, 0x1a, 0x40, 0x6a, 0x90, 0x6a, 0x90, 0x6a, 0x90, 0x6a, 0x90, 0x6a, 0x90, 0x55, 0x50];
const TOMBSTONE_DRAW: u16 = 0x2320;

const ARROW_WIDTH: u32 = 8;
const ARROW_HEIGHT: u32 = 8;
const ARROW_FLAGS: u32 = BLIT_1BPP;
const ARROW: [u8; 8] = [0x00, 0x18, 0x30, 0x60, 0xff, 0x60, 0x30, 0x18];
const ARROW_DRAW: u16 = 0x4440;

const DIAG_ARROW_WIDTH: u32 = 8;
const DIAG_ARROW_HEIGHT: u32 = 8;
const DIAG_ARROW_FLAGS: u32 = BLIT_1BPP;
const DIAG_ARROW: [u8; 8] = [0x1f, 0x03, 0x05, 0x09, 0x11, 0x20, 0x40, 0x80];
const DIAG_ARROW_DRAW: u16 = 0x4440;

const BOSS_WIDTH: u32 = 8;
const BOSS_HEIGHT: u32 = 16;
const BOSS_FLAGS: u32 = BLIT_2BPP;
const BOSS: [u8; 32] = [0x14, 0x14, 0x04, 0x10, 0x00, 0x00, 0x2f, 0xf8, 0x3b, 0xec, 0x3f, 0xfc, 0x3b, 0xec, 0x2f, 0xf8, 0x00, 0x00, 0x40, 0x01, 0x50, 0x05, 0x50, 0x05, 0x54, 0x15, 0x50, 0x05, 0x41, 0x41, 0x05, 0x50];
const BOSS_DRAW: u16 = 0x3402;

const ORBITING_SHOOTER_WIDTH: u32 = 8;
const ORBITING_SHOOTER_HEIGHT: u32 = 8;
const ORBITING_SHOOTER_FLAGS: u32 = BLIT_2BPP;
const ORBITING_SHOOTER: [u8; 16] = [0x05, 0x50, 0x1a, 0xa4, 0x6a, 0xa9, 0x6b, 0xe9, 0x6b, 0xe9, 0x6a, 0xa9, 0x1a, 0xa4, 0x05, 0x50];
const ORBITING_SHOOTER_DRAW: u16 = 0x4320;

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    pub data: &'static [u8],
    pub draw: u16,
}

macro_rules! make_sprite {
    ($sprite_name:ident, $id:ident) => {
        pub const $sprite_name: Sprite = Sprite { width: concat_idents!($id,_WIDTH), height: concat_idents!($id,_HEIGHT), flags: concat_idents!($id,_FLAGS), data: &$id, draw: concat_idents!($id,_DRAW) };
    }
}

make_sprite!(DANGO_SPRITE, DANGO);
make_sprite!(DANGO_DASH_SPRITE, DANGO_DASH);
make_sprite!(GRASS_SPRITE, GRASS);
make_sprite!(DANGO_EYE_SPRITE, DANGO_EYE);
make_sprite!(BULLET_SPRITE, BULLET);
make_sprite!(SPITWORM_SPRITE, SPITWORM);
make_sprite!(FLY_SPRITE, FLY);
make_sprite!(ROBOT_SPRITE, ROBOT);
make_sprite!(TOMBSTONE_SPRITE, TOMBSTONE);
make_sprite!(ARROW_SPRITE, ARROW);
make_sprite!(DIAG_ARROW_SPRITE, DIAG_ARROW);
make_sprite!(BOSS_SPRITE, BOSS);
make_sprite!(ORBITING_SHOOTER_SPRITE, ORBITING_SHOOTER);
