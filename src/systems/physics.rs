use amethyst::{
  core::Transform,
  derive::SystemDesc,
  ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::{
  components::{RigidBody, Velocity},
  resources::WorldGravity,
};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
  type SystemData = (
    ReadStorage<'s, Transform>,
    WriteStorage<'s, Velocity>,
    WriteStorage<'s, RigidBody>,
    Read<'s, WorldGravity>,
  );

  fn run(
    &mut self,
    (transforms, mut velocities, mut rigid_bodies, world_gravity): Self::SystemData,
  ) {
    for (transform, velocity, rigid_body) in
      (&transforms, &mut velocities, &mut rigid_bodies).join()
    {
      // TODO: Check if grounded
      rigid_body.is_grounded = transform.translation().y <= 150.0;

      if rigid_body.is_grounded && velocity.0.y < 0.0 {
        velocity.0.y = 0.0;
      }

      if rigid_body.use_gravity && !rigid_body.is_grounded {
        velocity.0.y -= world_gravity.0;
      }

      if velocity.0.x != 0.0 {
        if rigid_body.immediate_stop {
          velocity.0.x = 0.0;
        } else {
          if velocity.0.x > 0.0 {
            velocity.0.x -= rigid_body.drag;
            if velocity.0.x < 0.0 {
              velocity.0.x = 0.0;
            }
          } else {
            velocity.0.x += rigid_body.drag;
            if velocity.0.x > 0.0 {
              velocity.0.x = 0.0;
            }
          }
        }
      }
    }
  }
}
