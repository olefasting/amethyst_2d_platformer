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

pub mod animation;
pub mod collision_event;
pub mod components;
pub mod resources;
pub mod states;
pub mod systems;
pub mod utils;

use collision_event::{CollisionEvent, CollisionEventChannel};

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

    let game_data = GameDataBuilder::default()
        .with_bundle(rendering_bundle)?
        .with_bundle(input_bundle)?
        .with_bundle(TransformBundle::new())?
        .with(
            PlayerInputSystem::new(),
            "player_input_system",
            &["input_system"],
        )
        .with(ControlSystem, "control_system", &["player_input_system"])
        .with(CollisionSystem, "collision_system", &["control_system"])
        .with(PhysicsSystem, "physics_system", &["collision_system"])
        .with(MovementSystem, "movement_system", &["physics_system"])
        .with(AnimationSystem, "animation_system", &["movement_system"])
        .with(
            CameraFollowSystem,
            "camera_follow_system",
            &["movement_system"],
        );

    let mut game = Application::new(assets_dir, GameplayState::default(), game_data)?;
    game.run();

    Ok(())
}
