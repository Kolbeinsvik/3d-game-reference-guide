use crate::entities::{spawn_camera, spawn_cone, spawn_cube, spawn_plane, spawn_sphere};
use crate::prefabs::ui::set_up_ui;
use amethyst::input::is_key_down;
use amethyst::input::VirtualKeyCode;
use amethyst::prelude::*;
use amethyst::renderer::camera::ActiveCamera;
use amethyst::{GameData, SimpleState, StateData};

pub struct SetupState {}

impl Default for SetupState {
    fn default() -> Self {
        return Self {};
    }
}

impl SimpleState for SetupState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        spawn_plane(data.world);
        spawn_cube(data.world);
        spawn_sphere(data.world);
        spawn_cone(data.world);

        let camera = spawn_camera(data.world);
        data.world.insert(ActiveCamera {
            entity: Some(camera),
        });

        set_up_ui(data.world);

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
