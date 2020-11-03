//! Pong Tutorial 1

mod pong;
mod systems;

use crate::pong::Pong;
use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config_path = config_dir.join("bindings.ron");

    let transform_bundle = TransformBundle::default();
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config_path)?;

    let render_to_window_plugin =
        RenderToWindow::from_config_path(display_config_path)?.with_clear([0, 0, 0, 1]);
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(render_to_window_plugin)
        .with_plugin(RenderFlat2D::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with_bundle(rendering_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}
