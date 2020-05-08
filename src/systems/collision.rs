use amethyst::{
  core::Transform,
  derive::SystemDesc,
  ecs::{Join, ReadStorage, System, SystemData, Write},
};

use crate::{components::Collider, CollisionEventChannel};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
  type SystemData = (
    ReadStorage<'s, Transform>,
    ReadStorage<'s, Collider>,
    Write<'s, CollisionEventChannel>,
  );

  fn run(&mut self, (transforms, colliders, mut _collision_event_channel): Self::SystemData) {
    for (_transform, _collider) in (&transforms, &colliders).join() {}
  }
}
