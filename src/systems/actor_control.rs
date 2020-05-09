use amethyst::{
  core::math::Vector3,
  derive::SystemDesc,
  ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData},
  input::{InputHandler, StringBindings},
};

use amethyst_physics::prelude::*;

use crate::components::{actor::ActorData, ControlState, PlayerActor};

#[derive(SystemDesc)]
pub struct ActorControlSystem;

impl<'s> System<'s> for ActorControlSystem {
  type SystemData = (
    ReadStorage<'s, PlayerActor>,
    ReadStorage<'s, ActorData>,
    ReadStorage<'s, ControlState>,
    ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
    ReadExpect<'s, PhysicsWorld<f32>>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(
    &mut self,
    (player_actors, actor_datas, control_states, rigid_body_tags, physics_world, input): Self::SystemData,
  ) {
    for (_, _actor_data, control_state, rigid_body_tag) in (
      &player_actors,
      &actor_datas,
      &control_states,
      &rigid_body_tags,
    )
      .join()
    {
      let mut input_direction = Vector3::new(0.0, 0.0, 0.0);
      if control_state.up {
        input_direction.y += 1.0;
      }
      if control_state.down {
        input_direction.y -= 1.0
      }
      if control_state.right {
        input_direction.x += 1.0;
      }
      if control_state.left {
        input_direction.x -= 1.0;
      }

      if control_state.jump {
        // jump
      }

      physics_world
        .rigid_body_server()
        .apply_force(rigid_body_tag.get(), &(input_direction * 10.0));
      break;
    }
  }
}
