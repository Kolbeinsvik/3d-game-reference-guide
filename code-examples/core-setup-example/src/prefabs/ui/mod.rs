pub mod build_ui_prefabs;

use amethyst::prelude::World;
use amethyst::ui::UiCreator;
use crate::helpers::asset_helpers::generated_assets_prefabs_path;


pub fn set_up_ui(world: &mut World) {
    let path = generated_assets_prefabs_path().join("ui").join("ui.ron");
    world.exec(|mut creator: UiCreator<'_>| {
        creator.create(path.to_string_lossy(), ());
    });
    return ();
}