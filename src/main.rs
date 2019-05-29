extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    LogLevelFilter::Warn,
    LoggerConfig, StdoutLog,
};

mod rogue;
use rogue::Rogue;

mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        stdout: StdoutLog::Colored,
        level_filter: Warn,
        log_file: None,
        allow_env_override: true,
    });

    let path = "./resources/display_config.ron";

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0, 1.0, 1.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file("./resources/bindings_config.ron")?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[])
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PlayerSystem, "player_system", &["input_system"]);
    let mut game = Application::new("./", Rogue, game_data)?;

    game.run();

    Ok(())
}
