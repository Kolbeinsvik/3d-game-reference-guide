use amethyst::input::is_key_down;
use amethyst::input::VirtualKeyCode;
use amethyst::prelude::*;
use amethyst::renderer::camera::ActiveCamera;
use amethyst::{GameData, SimpleState, StateData};

use crate::entities::camera::spawn_camera_first_person;
use crate::entities::{spawn_cone, spawn_cube, spawn_plane, spawn_sphere};
use crate::prefabs::ui::set_up_ui;
use crate::resources::movement::insert_active_movement_resource;

pub struct SetupState {}

impl Default for SetupState {
    fn default() -> Self {
        return Self {};
    }
}

impl SimpleState for SetupState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        spawn_plane(world);
        spawn_cube(world);
        spawn_sphere(world);
        spawn_cone(world);

        let camera_first_person = spawn_camera_first_person(world);

        world.insert(ActiveCamera {
            entity: Some(camera_first_person),
        });

        insert_active_movement_resource(world);

        set_up_ui(world);

        return ();
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
