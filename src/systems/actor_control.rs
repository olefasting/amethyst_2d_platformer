use amethyst::{
  core::math::Vector3,
  derive::SystemDesc,
  ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

use amethyst_physics::prelude::*;

use crate::{
  components::{
    actor::{actions::*, ActorData},
    ControlState, PlayerActor,
  },
  resources::WorldGravity,
};

#[derive(SystemDesc)]
pub struct ActorControlSystem;

impl<'s> System<'s> for ActorControlSystem {
  type SystemData = (
    ReadStorage<'s, PlayerActor>,
    WriteStorage<'s, ActorData>,
    ReadStorage<'s, ControlState>,
    ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
    ReadExpect<'s, WorldGravity>,
    ReadExpect<'s, PhysicsWorld<f32>>,
  );

  fn run(
    &mut self,
    (
      player_actors,
      mut actor_datas,
      control_states,
      rigid_body_tags,
      world_gravity,
      physics_world,
    ): Self::SystemData,
  ) {
    for (_, actor_data, control_state, rigid_body_tag) in (
      &player_actors,
      &mut actor_datas,
      &control_states,
      &rigid_body_tags,
    )
      .join()
    {
      if let BodyMode::Kinematic = physics_world.rigid_body_server().mode(rigid_body_tag.get()) {
        let is_grounded = true;

        let mut velocity = physics_world
          .rigid_body_server()
          .linear_velocity(rigid_body_tag.get());

        if is_grounded {
          actor_data.jump_cnt = 0;
          actor_data.current_action = ACTION_IDLE;
        }

        if control_state.jump && actor_data.jump_cnt < actor_data.max_jump_cnt {
          actor_data.jump_cnt += 1;
          velocity.y = actor_data.jump_power;
          actor_data.current_action = ACTION_JUMP;
        }

        let max_speed_x = if is_grounded {
          actor_data.ground_max_speed
        } else {
          actor_data.air_max_speed
        };

        if control_state.left && !control_state.right {
          if is_grounded {
            velocity.x -= actor_data.ground_acceleration
          } else {
            velocity.x -= actor_data.air_acceleration
          }
        } else if control_state.right && !control_state.left {
          if is_grounded {
            velocity.x += actor_data.ground_acceleration
          } else {
            velocity.x += actor_data.air_acceleration
          }
        } else {
          velocity.x -= actor_data.drag;
        }

        if velocity.x > max_speed_x {
          velocity.x = max_speed_x;
        } else if velocity.x < -max_speed_x {
          velocity.x = -max_speed_x;
        } else {
          velocity.x = velocity.x;
        }

        if velocity.x > 0.0 {
          actor_data.facing_right = true;
          if is_grounded {
            actor_data.current_action = ACTION_RUN;
          }
        } else if velocity.x < 0.0 {
          actor_data.facing_right = false;
          if is_grounded {
            actor_data.current_action = ACTION_RUN;
          }
        }

        if velocity.y > actor_data.jump_power {
          velocity.y += actor_data.jump_power
        }

        if velocity.x < world_gravity.0.x {
          velocity.x = world_gravity.0.x
        }
        if velocity.y < world_gravity.0.y {
          velocity.y = world_gravity.0.y
        }
        if velocity.z < world_gravity.0.z {
          velocity.z = world_gravity.0.z
        }

        physics_world
          .rigid_body_server()
          .set_linear_velocity(rigid_body_tag.get(), &velocity);
      }
    }
  }
}