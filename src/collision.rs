use std::fmt;

use amethyst::ecs::world::Entity;

use ncollide2d::shape::ShapeHandle;

#[derive(Debug, Copy, Clone)]
pub enum CollisionDirection {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone)]
pub struct Collision {
  pub source_entity: Entity,
  pub source_collider: ShapeHandle<f32>,
  pub target_entity: Entity,
  pub target_collider: ShapeHandle<f32>,
  pub direction: CollisionDirection,
  pub is_ground: bool,
}

impl fmt::Debug for Collision {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("CollisionEvent")
      .field("source_entity", &self.source_entity)
      .field("target_entity", &self.target_entity)
      .field("direction", &self.direction)
      .field("is_ground", &self.is_ground)
      .finish()
  }
}

impl Collision {
  pub fn new(
    source_entity: Entity,
    source_collider: ShapeHandle<f32>,
    target_entity: Entity,
    target_collider: ShapeHandle<f32>,
    direction: CollisionDirection,
    is_ground: bool,
  ) -> Self {
    Self {
      source_entity,
      source_collider,
      target_entity,
      target_collider,
      direction,
      is_ground,
    }
  }
}
