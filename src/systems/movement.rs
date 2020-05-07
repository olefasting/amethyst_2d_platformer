use amethyst::{
  core::Transform,
  derive::SystemDesc,
  ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
  type SystemData = (ReadStorage<'s, Velocity>, WriteStorage<'s, Transform>);

  fn run(&mut self, (velocities, mut transforms): Self::SystemData) {
    for (velocity, transform) in (&velocities, &mut transforms).join() {
      transform.prepend_translation(velocity.0);
    }
  }
}
