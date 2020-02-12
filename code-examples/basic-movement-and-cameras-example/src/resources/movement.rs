use crate::prefabs::ui::context::ContextLabelId;
use amethyst::prelude::*;
use amethyst::ui::UiFinder;

pub enum MovementType {
    Translational,
    FreeFloat,
}

pub struct ActiveMovement {
    pub movement_type: MovementType,
}

impl Default for ActiveMovement {
    fn default() -> Self {
        Self {
            movement_type: MovementType::Translational,
        }
    }
}

pub fn insert_active_movement_resource(world: &mut World) {
    world.insert(ActiveMovement::default())
}
