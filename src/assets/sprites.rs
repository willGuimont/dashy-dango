use std::concat_idents;

use crate::{BLIT_1BPP, BLIT_2BPP};

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
const DANGO_EYE: [u8; 1] = [
    0b10110100,
];
const DANGO_EYE_DRAW: u16 = 0x0020;

const BULLET_WIDTH: u32 = 2;
const BULLET_HEIGHT: u32 = 2;
const BULLET_FLAGS: u32 = BLIT_1BPP;
const BULLET: [u8; 1] = [
    0b11110000,
];
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
