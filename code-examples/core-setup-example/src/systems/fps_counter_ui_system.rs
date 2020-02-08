use amethyst::ecs::prelude::*;
use amethyst::ecs::System;
use amethyst::ui::UiFinder;
use amethyst::ui::UiText;
use amethyst::utils::fps_counter::FpsCounter;
use amethyst::core::Time;

pub struct FpsCounterUiSystem {
    pub fps_display: Option<Entity>,
}

impl Default for FpsCounterUiSystem {
    fn default() -> Self {
        return Self {
            fps_display: None
        };
    }
}

impl<'a> FpsCounterUiSystem {
    pub const NAME: &'a str = "fps_counter_ui_system";
}

impl<'a> System<'a> for FpsCounterUiSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
        UiFinder<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            time,
            mut ui_text,
            fps_counter,
            finder
        ) = data;


        if self.fps_display.is_none() {
            if let Some(fps_entity) = finder.find("fps") {
                self.fps_display = Some(fps_entity);
            }
        }
        if let Some(fps_entity) = self.fps_display {
            if let Some(fps_display) = ui_text.get_mut(fps_entity) {
                if time.frame_number() % 20 == 0 {
                    let fps = fps_counter.sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }

        return ();
    }
}