use std::concat_idents;

use crate::{BLIT_1BPP, BLIT_2BPP};

const DANGO_WIDTH: u32 = 8;
const DANGO_HEIGHT: u32 = 8;
const DANGO_FLAGS: u32 = BLIT_1BPP;
const DANGO: [u8; 8] = [0x00, 0x00, 0x3c, 0x7e, 0x7e, 0x7e, 0x7e, 0x00];
const DANGO_DRAW: u16 = 0x0030;

const DANGO_OUTLINE_WIDTH: u32 = 8;
const DANGO_OUTLINE_HEIGHT: u32 = 8;
const DANGO_OUTLINE_FLAGS: u32 = BLIT_1BPP;
const DANGO_OUTLINE: [u8; 8] = [0x00, 0x3c, 0x42, 0x81, 0x81, 0x81, 0x81, 0x7e];
const DANGO_OUTLINE_DRAW: u16 = 0x0040;

const DANGO_DASH_OUTLINE_WIDTH: u32 = 8;
const DANGO_DASH_OUTLINE_HEIGHT: u32 = 8;
const DANGO_DASH_OUTLINE_FLAGS: u32 = BLIT_1BPP;
const DANGO_DASH_OUTLINE: [u8; 8] = DANGO_OUTLINE;
const DANGO_DASH_OUTLINE_DRAW: u16 = 0x0030;

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
make_sprite!(GRASS_SPRITE, GRASS);
make_sprite!(DANGO_EYE_SPRITE, DANGO_EYE);
make_sprite!(DANGO_OUTLINE_SPRITE, DANGO_OUTLINE);
make_sprite!(DANGO_DASH_OUTLINE_SPRITE, DANGO_DASH_OUTLINE);
