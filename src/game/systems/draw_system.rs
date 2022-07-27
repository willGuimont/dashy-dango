use ecs_macro::Component;
use crate::*;
use crate::{blit, BLIT_1BPP, CameraComponent, entities_with_components,has_all_components,entities_with,get_components_unwrap, PositionComponent, Registry};
use crate::ecs::Entity;

const SCREEN_CENTER: (f32, f32) = (76.0, 76.0);

//FIXME add spriteComponent
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

pub fn draw_entity(registry:&Registry){
    for (e,(cam, camPos)) in entities_with_components!(registry,CameraComponent, PositionComponent){
        for (e,(pos,)) in entities_with_components!(registry, PositionComponent){
            let new_pos = camera_conversion(pos, camPos);
            blit(&SMILEY, new_pos.0, new_pos.1, 8, 8, BLIT_1BPP);
        }
    }
}

fn camera_conversion(pos:&PositionComponent, camPos:&PositionComponent) -> (i32, i32) {
    unsafe { ((pos.x - camPos.x + SCREEN_CENTER.0) as i32, (pos.y - camPos.y + SCREEN_CENTER.1) as i32) }
}
