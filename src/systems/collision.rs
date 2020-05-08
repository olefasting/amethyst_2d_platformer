use amethyst::{
  core::Transform,
  derive::SystemDesc,
  ecs::{Entities, Join, ReadStorage, System, SystemData, Write},
};

use crate::{
  components::{Collider, Ground},
  resources::Collisions,
  Collision, CollisionDirection,
};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Transform>,
    ReadStorage<'s, Collider>,
    ReadStorage<'s, Ground>,
    Write<'s, Collisions>,
  );

  fn run(&mut self, (entities, transforms, colliders, grounds, mut collisions): Self::SystemData) {
    collisions.0.clear();

    for (e, transform, collider, ()) in (&entities, &transforms, &colliders, !&grounds).join() {
      if let Some(shape) = collider.shape_handle.as_ref() {
        let position = transform.translation();

        let threshold_x = collider.distance_threshold.x;
        let threshold_y = collider.distance_threshold.y;

        for (other_e, other_transform, other_collider, ground) in
          (&entities, &transforms, &colliders, (&grounds).maybe()).join()
        {
          if let Some(other_shape) = other_collider.shape_handle.as_ref() {
            let other_position = other_transform.translation();

            let x_dist = other_position.x - position.x;
            let y_dist = other_position.y - position.y;

            if x_dist <= threshold_x
              && x_dist >= -threshold_x
              && y_dist <= threshold_y
              && y_dist >= -threshold_y
            {
              let is_ground = match ground {
                Some(_) => true,
                None => false,
              };

              let collision = true;
              if collision {
                let direction = CollisionDirection::Down;
                let event = Collision::new(
                  e,
                  shape.clone(),
                  other_e,
                  other_shape.clone(),
                  direction,
                  is_ground,
                );

                collisions.0.push(event);
              }
            }
          }
        }
      }
    }
  }
}
