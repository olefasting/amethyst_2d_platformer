use amethyst::{
  core::math::Vector3,
  derive::SystemDesc,
  ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData},
  input::{InputHandler, StringBindings},
};

use amethyst_physics::prelude::*;

use crate::components::{actor::ActorData, PlayerActor};

#[derive(SystemDesc)]
pub struct ActorControlSystem {
  jump_held: bool,
}

impl Default for ActorControlSystem {
  fn default() -> Self {
    Self { jump_held: false }
  }
}

impl<'s> System<'s> for ActorControlSystem {
  type SystemData = (
    ReadStorage<'s, PlayerActor>,
    ReadStorage<'s, ActorData>,
    ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
    ReadExpect<'s, PhysicsTime>,
    ReadExpect<'s, PhysicsWorld<f32>>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(
    &mut self,
    (
      player_actors,
      actor_datas,
      rigid_body_tags,
      physics_time,
      physics_world,
      input,
    ): Self::SystemData,
  ) {
    let mut input_direction: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

    let jump = if input.action_is_down("jump").unwrap_or(false) {
      if self.jump_held {
        false
      } else {
        self.jump_held = true;
        true
      }
    } else {
      self.jump_held = false;
      false
    };

    // input.action_is_down("up").unwrap_or(false);
    // input.action_is_down("down").unwrap_or(false);
    if input.action_is_down("right").unwrap_or(false) {
      input_direction.x += 1.0;
    }
    if input.action_is_down("left").unwrap_or(false) {
      input_direction.x -= 1.0;
    }

    for (_, _actor_data, rigid_body_tag) in (&player_actors, &actor_datas, &rigid_body_tags).join()
    {
      physics_world
        .rigid_body_server()
        .apply_force(rigid_body_tag.get(), &(input_direction * 10.0));

      let vel = physics_world
        .rigid_body_server()
        .linear_velocity(rigid_body_tag.get());
      if vel.x != 0.0 {
        dbg!(vel);
      }

      break;
    }
  }
}
