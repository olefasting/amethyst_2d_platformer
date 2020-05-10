use amethyst::{
  core::math::Vector3,
  prelude::*,
  renderer::debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
};

use amethyst_physics::prelude::*;

use crate::{components::*, entities::*, resources::*};

const WORLD_GRAVITY: f32 = 64.0;
const WORLD_TERMINAL_VELOCITY: f32 = 300.0;

#[derive(Debug, Default)]
pub struct GameplayState;

impl SimpleState for GameplayState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    world.register::<LocalPlayer>();
    world.register::<Controllable>();
    world.register::<ControlState>();
    world.register::<AnimatedSprite>();
    world.register::<DebugShape>();

    world.register::<DebugLinesComponent>();

    world.insert(DebugLines::new());
    world.insert(DebugLinesParams { line_width: 1.0 });

    world.insert(CurrentState(StateId::GameplayState));

    let gravity_vec = Vector3::new(0.0, -WORLD_GRAVITY, 0.0);
    world.insert(WorldGravity(gravity_vec));
    world.insert(WorldTerminalVelocity(Vector3::new(
      WORLD_TERMINAL_VELOCITY,
      WORLD_TERMINAL_VELOCITY,
      WORLD_TERMINAL_VELOCITY,
    )));

    setup_physics(world, &gravity_vec);

    let _player = create_player(world);

    let camera = create_camera(world);

    world.insert(ActiveCamera(camera));

    create_level(world);
  }

  fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    Trans::None
  }
}

fn setup_physics(world: &mut World, _gravity_vec: &Vector3<f32>) {
  let physics_world = world.fetch::<PhysicsWorld<f32>>();
  // Not using nphysics gravity until collisions are sorted out, so
  // that player can be a kinematic body
  //
  // physics_world.world_server().set_gravity(gravity_vec);
  let gravity_vec = Vector3::new(0.0, 0.0, 0.0);
  physics_world.world_server().set_gravity(&gravity_vec);
}
