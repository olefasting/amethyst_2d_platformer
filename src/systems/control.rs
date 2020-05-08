use amethyst::{
  derive::SystemDesc,
  ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::{
  components::{
    actor::{actions::*, ActorData, ControlState},
    PhysicsBody, Velocity,
  },
  resources::WorldGravity,
};

#[derive(SystemDesc)]
pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
  type SystemData = (
    WriteStorage<'s, Velocity>,
    WriteStorage<'s, ActorData>,
    ReadStorage<'s, PhysicsBody>,
    WriteStorage<'s, ControlState>,
    Read<'s, WorldGravity>,
  );

  fn run(
    &mut self,
    (mut velocities, mut actor_datas, physics_bodies, mut control_states, world_gravity): Self::SystemData,
  ) {
    for (velocity, actor_data, physics_body, control_state) in (
      &mut velocities,
      &mut actor_datas,
      &physics_bodies,
      &mut control_states,
    )
      .join()
    {
      if physics_body.is_grounded {
        actor_data.jump_cnt = 0;
        actor_data.current_action = ACTION_IDLE;
      }

      if control_state.jump && actor_data.jump_cnt < actor_data.max_jump_cnt {
        actor_data.jump_cnt += 1;
        velocity.0.y = actor_data.jump_power;
        actor_data.current_action = ACTION_JUMP;
      }

      let max_speed_x = if physics_body.is_grounded {
        actor_data.ground_max_speed
      } else {
        actor_data.air_max_speed
      };

      velocity.0.x = if control_state.left && !control_state.right {
        if physics_body.immediate_start {
          if physics_body.is_grounded {
            -actor_data.ground_max_speed
          } else {
            -actor_data.air_max_speed
          }
        } else {
          if physics_body.is_grounded {
            velocity.0.x - actor_data.ground_acceleration
          } else {
            velocity.0.x - actor_data.air_acceleration
          }
        }
      } else if control_state.right && !control_state.left {
        if physics_body.immediate_start {
          if physics_body.is_grounded {
            actor_data.ground_max_speed
          } else {
            actor_data.air_max_speed
          }
        } else {
          if physics_body.is_grounded {
            velocity.0.x + actor_data.ground_acceleration
          } else {
            velocity.0.x + actor_data.air_acceleration
          }
        }
      } else {
        if physics_body.immediate_stop {
          0.0
        } else {
          velocity.0.x
        }
      };

      velocity.0.x = if velocity.0.x > max_speed_x {
        max_speed_x
      } else if velocity.0.x < -max_speed_x {
        -max_speed_x
      } else {
        velocity.0.x
      };

      if velocity.0.x > 0.0 {
        actor_data.facing_right = true;
        if physics_body.is_grounded {
          actor_data.current_action = ACTION_RUN;
        }
      } else if velocity.0.x < 0.0 {
        actor_data.facing_right = false;
        if physics_body.is_grounded {
          actor_data.current_action = ACTION_RUN;
        }
      }

      velocity.0.y = if velocity.0.y > actor_data.jump_power {
        actor_data.jump_power
      } else if velocity.0.y < -world_gravity.0 {
        -world_gravity.0
      } else {
        velocity.0.y
      };

      control_state.reset();
    }
  }
}
