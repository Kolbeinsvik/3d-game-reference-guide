use crate::prefabs::ui::context::ContextLabelId;
use crate::resources::ui_context::UiContextResource;
use amethyst::core::Time;
use amethyst::ecs::prelude::*;
use amethyst::ecs::System;
use amethyst::ui::UiFinder;
use amethyst::ui::UiText;
use amethyst::utils::fps_counter::FpsCounter;

pub struct CharacterSwitcherSystem;

impl<'a> CharacterSwitcherSystem {
    pub const NAME: &'a str = "character_switcher_system";
}

impl<'a> System<'a> for CharacterSwitcherSystem {
    type SystemData = (Read<'a, UiContextResource>,);

    fn run(&mut self, data: Self::SystemData) {
        let (context_resource) = data;

        return ();
    }
}
