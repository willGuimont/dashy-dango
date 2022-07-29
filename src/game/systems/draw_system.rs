use crate::*;
use crate::{blit, BLIT_1BPP, CameraComponent, entities_with, entities_with_components, get_components_unwrap, has_all_components, PositionComponent, Registry};
use crate::ecs::Entity;
use crate::game::system::System;

const SCREEN_CENTER: (f32, f32) = (76.0, 76.0);

//TODO add spriteComponent
#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];


pub struct DrawSystem {}

impl System for DrawSystem {
    fn execute_system(&self, registry: &mut Registry) -> () {
        for (_, (_, cam_pos)) in entities_with_components!(registry, CameraComponent, PositionComponent) {
            for (_, (pos, )) in entities_with_components!(registry, PositionComponent) {
                let new_pos = camera_conversion(pos, cam_pos);
                blit(&SMILEY, new_pos.0, new_pos.1, 8, 8, BLIT_1BPP);
            }
        }
    }
}

impl DrawSystem {
    pub fn new() -> Self { DrawSystem {} }
}

fn camera_conversion(pos: &PositionComponent, cam_pos: &PositionComponent) -> (i32, i32) {
    ((pos.x - cam_pos.x + SCREEN_CENTER.0) as i32, (pos.y - cam_pos.y + SCREEN_CENTER.1) as i32)
}
