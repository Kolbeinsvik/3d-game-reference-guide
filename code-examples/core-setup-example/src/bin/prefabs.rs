use core_setup_example::prefabs::ui::build_ui_prefabs::build_ui_prefabs;
use std::fs::create_dir_all;
use std::io::prelude::*;
use amethyst::utils::application_root_dir;

fn main() {
    println!("Buildings prefabs");
    let app_root = application_root_dir().unwrap();
    let assets_path = app_root.join("generated").join("assets").join("prefabs");

    let ui_dir_path = assets_path.join("ui");
    if ui_dir_path.exists() == false {
        create_dir_all(ui_dir_path).unwrap();
    }

    let s = build_ui_prefabs();
    let ui_path = assets_path.join("ui").join("ui.ron");


    let mut file = std::fs::File::create(ui_path).unwrap();
    file.write(s.as_bytes()).unwrap();
}