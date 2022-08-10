use crate::{Abort, entities_with, get_components_clone_unwrap, has_all_components, Registry, Vec2};
use crate::assets::{DANGO_EYE_SPRITE, DANGO_SPRITE};
use crate::ecs::Entity;
use crate::game::components::{ChildComponent, DangoEyeComponent, GamepadComponent, PositionComponent};
use crate::game::systems::System;
use crate::utils::gamepad_to_vec;

pub struct DangoEyesSystem;

impl System for DangoEyesSystem {
    fn execute_system(&mut self, registry: &mut Registry) {
        for e in entities_with!(registry, DangoEyeComponent, ChildComponent, PositionComponent) {
            let (gamepad, mut child) = get_components_clone_unwrap!(registry, e, GamepadComponent, ChildComponent);
            let direction = gamepad_to_vec(gamepad.get_gamepad());
            let offset = Vec2::new(
                (DANGO_SPRITE.width - DANGO_EYE_SPRITE.width) as f32 / 2.0 + 1.0,
                (DANGO_SPRITE.height - DANGO_EYE_SPRITE.height) as f32 / 2.0 + 1.0,
            );
            let scaling = 1.5;

            let mut direction = scaling * direction;
            if direction.x > 1.0 {
                direction.x = 1.0;
            }
            if direction.y < -1.0 {
                direction.y = -1.0;
            }
            if direction.x.abs() >= 1.0 && direction.y.abs() >= 1.0 {
                direction.y = 1.0 * direction.y.signum();
            }

            child.relative_pos = direction + offset;

            registry.add_component(e, child);
        }
    }
}
