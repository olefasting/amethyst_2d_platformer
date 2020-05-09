use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use amethyst_physics::PhysicsBundle;

use amethyst_nphysics::NPhysicsBackend;

pub mod animation;
pub mod components;
pub mod resources;
pub mod states;
pub mod systems;
pub mod utils;

use animation::*;
// use components::*;
// use resources::*;
use states::*;
use systems::*;
// use utils::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    let display_config_path = config_dir.join("display.ron");
    let bindings_path = app_root.join("config").join("bindings.ron");

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default());

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?;

    let physics_bundle = PhysicsBundle::<f32, NPhysicsBackend>::new()
        .with_bundle_pre_physics(input_bundle)
        .with_pre_physics(
            ActorControlSystem::default(),
            String::from("actor_control_system"),
            vec![String::from("input_system")],
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(physics_bundle)?
        .with_bundle(rendering_bundle)?
        .with(AnimationSystem, "animation_system", &[]);

    let mut game = Application::new(assets_dir, GameplayState::default(), game_data)?;
    game.run();

    Ok(())
}
