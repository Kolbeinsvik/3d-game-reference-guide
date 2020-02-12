use std::fs::create_dir_all;
use std::io::Write;

use amethyst::prelude::World;
use amethyst::ui::UiCreator;

use crate::helpers::asset_helpers::generated_assets_prefabs_path;
use crate::prefabs::ui::context::build_context_ui_prefab;
use crate::prefabs::ui::fps::build_fps_counter_ui_prefab;
use crate::prefabs::PrefabFilename;

pub mod context;
pub mod fps;

pub fn set_up_ui(world: &mut World) {
    let fps_path = generated_assets_prefabs_path()
        .join("ui")
        .join(PrefabFilename::UiFpsCounter.filename());
    let context_path = generated_assets_prefabs_path()
        .join("ui")
        .join(PrefabFilename::UiContext.filename());

    world.exec(|mut creator: UiCreator<'_>| {
        creator.create(fps_path.to_string_lossy(), ());
        creator.create(context_path.to_string_lossy(), ());
    });
    return ();
}

pub fn generate_ui_prefabs() {
    let assets_path = generated_assets_prefabs_path();

    let ui_dir_path = assets_path.join("ui");
    if ui_dir_path.exists() == false {
        create_dir_all(ui_dir_path.clone()).unwrap();
    }
    let ui_fps_path = ui_dir_path.join(PrefabFilename::UiFpsCounter.filename());
    let ui_context_path = ui_dir_path.join(PrefabFilename::UiContext.filename());

    let fps_counter_ui_prefab = build_fps_counter_ui_prefab();
    let ui_prefab_context = build_context_ui_prefab();

    std::fs::File::create(ui_fps_path)
        .unwrap()
        .write(fps_counter_ui_prefab.as_bytes())
        .unwrap();

    std::fs::File::create(ui_context_path)
        .unwrap()
        .write(ui_prefab_context.as_bytes())
        .unwrap();
}
