use std::fs::create_dir_all;
use std::io::Write;

use amethyst::prelude::World;
use amethyst::ui::UiCreator;

use crate::helpers::asset_helpers::generated_assets_prefabs_path;
use crate::prefabs::ui::fps::build_fps_counter_ui_prefab;
use crate::prefabs::PrefabFilename;

pub mod fps;

pub fn set_up_ui(world: &mut World) {
    let path = generated_assets_prefabs_path()
        .join("ui")
        .join(PrefabFilename::UiFpsCounter.filename());
    world.exec(|mut creator: UiCreator<'_>| {
        creator.create(path.to_string_lossy(), ());
    });
    return ();
}

pub fn generate_ui_prefabs() {
    let assets_path = generated_assets_prefabs_path();

    let ui_dir_path = assets_path.join("ui");
    if ui_dir_path.exists() == false {
        create_dir_all(ui_dir_path.clone()).unwrap();
    }
    let ui_path = ui_dir_path.join(PrefabFilename::UiFpsCounter.filename());

    let fps_counter_ui_prefab = build_fps_counter_ui_prefab();

    std::fs::File::create(ui_path)
        .unwrap()
        .write(fps_counter_ui_prefab.as_bytes())
        .unwrap();
}
