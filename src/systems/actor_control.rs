use amethyst::{
  core::{math::Vector3, Transform},
  derive::SystemDesc,
  ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

use amethyst_physics::prelude::*;

use crate::{
  components::{
    actor::{actions::*, ActorData},
    physics::COLLISION_GROUP_GROUND,
    ControlState, PlayerActor,
  },
  resources::WorldGravity,
};

#[derive(SystemDesc)]
pub struct ActorControlSystem;

impl<'s> System<'s> for ActorControlSystem {
  type SystemData = (
    ReadStorage<'s, PlayerActor>,
    ReadStorage<'s, Transform>,
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
      transforms,
      mut actor_datas,
      control_states,
      rigid_body_tags,
      world_gravity,
      physics_world,
    ): Self::SystemData,
  ) {
    for (_, transform, actor_data, control_state, rigid_body_tag) in (
      &player_actors,
      &transforms,
      &mut actor_datas,
      &control_states,
      &rigid_body_tags,
    )
      .join()
    {
      // if let BodyMode::Kinematic = physics_world.rigid_body_server().mode(rigid_body_tag.get()) {
      let mut velocity = physics_world
        .rigid_body_server()
        .linear_velocity(rigid_body_tag.get());

      let is_grounded = true;

      /*
      // FIXME:: This is a temp solution. Don't reallocate this
      let mut contact_events: Vec<ContactEvent<f32>> = Vec::with_capacity(1024);
      physics_world
        .rigid_body_server()
        .contact_events(rigid_body_tag.get(), &mut contact_events);

      for event in contact_events {
        for group in physics_world
          .rigid_body_server()
          .belong_to(event.other_body)
        {
          if group.get() == COLLISION_GROUP_GROUND {
            is_grounded = true;
          }
        }
      }
      if is_grounded {
        if velocity.y < 0.0 {
          velocity.y = 0.0;
        }
        actor_data.jump_cnt = 0;
        actor_data.current_action = ACTION_IDLE;
      } else {
          velocity += world_gravity.0;
        }
        */

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

      if control_state.left {
        if is_grounded {
          velocity.x -= actor_data.ground_acceleration
        } else {
          velocity.x -= actor_data.air_acceleration
        }
        println!("left: {}", velocity.x)
      } else if control_state.right {
        if is_grounded {
          velocity.x += actor_data.ground_acceleration
        } else {
          velocity.x += actor_data.air_acceleration
        }
        println!("right: {}", velocity.x)
      } else {
        if velocity.x > 0.0 {
          velocity.x -= actor_data.drag;
          if velocity.x < 0.0 {
            velocity.x = 0.0;
          }
        } else if velocity.x < 0.0 {
          velocity.x += actor_data.drag;
          if velocity.x > 0.0 {
            velocity.x = 0.0;
          }
        }
      }

      if velocity.x > max_speed_x {
        velocity.x = max_speed_x;
      } else if velocity.x < -max_speed_x {
        velocity.x = -max_speed_x;
      }

      if velocity.y > actor_data.jump_power {
        velocity.y += actor_data.jump_power
      }

      if velocity.x < world_gravity.0.x {
        velocity.x += world_gravity.0.x
      }
      if velocity.y < world_gravity.0.y {
        velocity.y += world_gravity.0.y
      }
      if velocity.z < world_gravity.0.z {
        velocity.z += world_gravity.0.z
      }

      if control_state.right {
        actor_data.facing_right = true;
        if is_grounded {
          actor_data.current_action = ACTION_RUN;
        }
      } else if control_state.left {
        actor_data.facing_right = false;
        if is_grounded {
          actor_data.current_action = ACTION_RUN;
        }
      } else if is_grounded {
        actor_data.current_action = ACTION_IDLE;
      } else {
        actor_data.current_action = ACTION_FALL;
      }

      physics_world
        .rigid_body_server()
        .set_linear_velocity(rigid_body_tag.get(), &velocity);
    }
  }
  // }
}
