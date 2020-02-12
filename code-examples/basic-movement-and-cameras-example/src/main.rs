use amethyst;
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::core::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::input::StringBindings;
use amethyst::prelude::*;
use amethyst::renderer::palette::Srgb;
use amethyst::renderer::plugins::RenderDebugLines;
use amethyst::renderer::plugins::RenderFlat3D;
use amethyst::renderer::plugins::RenderSkybox;
use amethyst::renderer::plugins::RenderToWindow;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::ui::RenderUi;
use amethyst::ui::UiBundle;
use amethyst::utils::application_root_dir;
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst::window::DisplayConfig;
use amethyst::GameDataBuilder;
use clap::Clap;

use crate::prefabs::ui::generate_ui_prefabs;
use crate::states::SetupState;
use crate::systems::fps_counter_ui_system::FpsCounterUiSystem;

mod entities;
mod helpers;
mod prefabs;
mod resources;
mod states;
mod systems;

#[derive(Clap)]
#[clap(version = "1.0", author = "Debuglines")]
struct Opts {
    #[clap(long = "prefabs-ui")]
    prefabs_ui: bool,
}

fn main() -> amethyst::Result<()> {
    handle_prefab_generation();
    return start_game();
}

fn start_game() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources_path = app_root.join("assets");
    let mut display_config = DisplayConfig::default();
    let game_title = "Basic movement and cameras example".to_owned();
    display_config.title = game_title;

    let input_bundle = InputBundle::<StringBindings>::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config).with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderFlat3D::default())
                .with_plugin(RenderSkybox::with_colors(
                    Srgb::new(0.82, 0.51, 0.50),
                    Srgb::new(0.18, 0.11, 0.85),
                ))
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with(FpsCounterUiSystem::default(), FpsCounterUiSystem::NAME, &[]);

    let mut game = Application::build(resources_path, SetupState::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 144)
        .build(game_data)?;

    game.run();

    Ok(())
}

fn handle_prefab_generation() {
    let opts: Opts = Opts::parse();
    if opts.prefabs_ui {
        println!("Generating ui prefabs");
        generate_ui_prefabs();
    }

    return ();
}
