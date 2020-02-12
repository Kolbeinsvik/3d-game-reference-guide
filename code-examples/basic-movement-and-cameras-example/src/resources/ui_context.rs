use crate::prefabs::ui::context::ContextLabelId;
use amethyst::core::ecs::Entity;
use amethyst::prelude::*;
use amethyst::ui::UiFinder;

pub struct UiContextResource {
    pub camera_first_person: Entity,
    pub camera_third_person: Entity,
    pub camera_isometric: Entity,
    pub camera_top_down: Entity,
    pub camera_side_scroller: Entity,
    pub movement_translational: Entity,
    pub movement_free_float: Entity,
}

impl Default for UiContextResource {
    fn default() -> Self {
        unimplemented!()
    }
}

pub fn insert_context_resource(world: &mut World) {
    let resource = world.exec(|ui_finder: UiFinder<'_>| {
        let camera_first_person = ui_finder
            .find(ContextLabelId::CameraFirstPerson.id())
            .unwrap();
        let camera_third_person = ui_finder
            .find(ContextLabelId::CameraThirdPerson.id())
            .unwrap();
        let camera_isometric = ui_finder
            .find(ContextLabelId::CameraIsometric.id())
            .unwrap();
        let camera_top_down = ui_finder.find(ContextLabelId::CameraTopDown.id()).unwrap();
        let camera_side_scroller = ui_finder
            .find(ContextLabelId::CameraSideScroller.id())
            .unwrap();
        let movement_translational = ui_finder
            .find(ContextLabelId::MovementTranslational.id())
            .unwrap();
        let movement_free_float = ui_finder
            .find(ContextLabelId::MovementFreeFloat.id())
            .unwrap();

        let resource = UiContextResource {
            camera_first_person,
            camera_third_person,
            camera_isometric,
            camera_top_down,
            camera_side_scroller,
            movement_translational,
            movement_free_float,
        };
        resource
    });

    world.insert(resource);
}
