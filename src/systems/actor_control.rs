use amethyst::{
  core::{math::Vector3, Transform},
  derive::SystemDesc,
  ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

use amethyst_physics::prelude::*;

use crate::{
  components::{
    actor::{actions::*, ActorData, ControlMode},
    physics::COLLISION_GROUP_GROUND,
    ControlState,
  },
  resources::{WorldGravity, WorldTerminalVelocity},
};

#[derive(SystemDesc)]
pub struct ActorControlSystem;

impl<'s> System<'s> for ActorControlSystem {
  type SystemData = (
    WriteStorage<'s, ActorData>,
    ReadStorage<'s, ControlState>,
    ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
    ReadExpect<'s, WorldGravity>,
    ReadExpect<'s, WorldTerminalVelocity>,
    ReadExpect<'s, PhysicsWorld<f32>>,
    ReadExpect<'s, PhysicsTime>,
  );

  fn run(
    &mut self,
    (
      mut actor_datas,
      control_states,
      rigid_body_tags,
      gravity,
      terminal_velocity,
      physics_world,
      physics_time,
    ): Self::SystemData,
  ) {
    for (actor_data, control_state, rigid_body_tag) in
      (&mut actor_datas, &control_states, &rigid_body_tags).join()
    {
      let mut velocity = physics_world
        .rigid_body_server()
        .linear_velocity(rigid_body_tag.get());

      let mut is_grounded = false;

      actor_data.contact_events_as_mut().clear();

      physics_world
        .rigid_body_server()
        .contact_events(rigid_body_tag.get(), actor_data.contact_events_as_mut());

      for event in actor_data.contact_events_as_mut() {
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
        actor_data.jump_cnt = 0;
        actor_data.current_action = ACTION_IDLE;
      } else {
        // TODO: Fix gravity and terminal velocity so that it works bi-directionally on all axes
        let new_y = velocity.y + gravity.0.y;
        if velocity.y > -terminal_velocity.0.y {
          if new_y < -terminal_velocity.0.y {
            velocity.y = -terminal_velocity.0.y;
          } else {
            velocity.y = new_y;
          }
        }
      }

      if control_state.jump && actor_data.jump_cnt < actor_data.max_jump_cnt {
        actor_data.jump_cnt += 1;
        velocity.y = actor_data.jump_power;
        actor_data.current_action = ACTION_JUMP;
      }

      if let ControlMode::Realistic = actor_data.control_mode {
        // Realistic movement
        if control_state.left {
          // Move left
          actor_data.facing_right = false;
          if is_grounded {
            let new_x = velocity.x - actor_data.ground_acceleration;
            velocity.x = if new_x < -actor_data.ground_max_speed {
              -actor_data.ground_max_speed
            } else {
              new_x
            };
          } else {
            let new_x = velocity.x - actor_data.air_acceleration;
            velocity.x = if new_x < -actor_data.air_max_speed {
              -actor_data.air_max_speed
            } else {
              new_x
            };
          }
        } else if control_state.right {
          // Move right
          actor_data.facing_right = true;
          if is_grounded {
            let new_x = velocity.x + actor_data.ground_acceleration;
            velocity.x = if new_x > actor_data.ground_max_speed {
              actor_data.ground_max_speed
            } else {
              new_x
            };
          } else {
            let new_x = velocity.x + actor_data.air_acceleration;
            velocity.x = if new_x > actor_data.air_max_speed {
              actor_data.air_max_speed
            } else {
              new_x
            };
          }
        } else {
          // No move
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
      } else {
        // Instant  movement
        if control_state.left {
          // Move left
          velocity.x = if is_grounded {
            -actor_data.ground_max_speed
          } else {
            -actor_data.air_max_speed
          }
        } else if control_state.right {
          // Move right
          velocity.x = if is_grounded {
            actor_data.ground_max_speed
          } else {
            actor_data.air_max_speed
          }
        } else {
          // No move
          velocity.x = 0.0;
        }
      }

      // Animation
      if is_grounded {
        if control_state.left || control_state.right {
          actor_data.facing_right = control_state.right;
          actor_data.current_action = ACTION_RUN;
        } else {
          actor_data.current_action = ACTION_IDLE;
        }
      } else {
        actor_data.current_action = ACTION_FALL;
      }

      physics_world
        .rigid_body_server()
        .set_linear_velocity(rigid_body_tag.get(), &velocity);
    }
  }
}
