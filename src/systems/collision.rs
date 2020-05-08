use amethyst::{
  core::Transform,
  derive::SystemDesc,
  ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Collider, RayTracer};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
  type SystemData = (
    ReadStorage<'s, Transform>,
    ReadStorage<'s, RayTracer>,
    ReadStorage<'s, Collider>,
  );

  fn run(&mut self, (transforms, ray_tracers, colliders): Self::SystemData) {
    for (transform, ray_tracer) in (&transforms, &ray_tracers).join() {}
    for (transform, collider) in (&transforms, &colliders).join() {}
  }
}
