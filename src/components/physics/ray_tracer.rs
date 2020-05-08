use amethyst::{
  core::math::Vector2,
  ecs::{Component, VecStorage},
};

use crate::Ray;

#[derive(Debug, Clone)]
pub struct RayTracer {
  pub colliders: Vec<Ray>,
  pub debug_draw: bool,
}

impl RayTracer {
  pub fn new(colliders: Vec<Ray>, debug_draw: bool) -> Self {
    Self {
      colliders,
      debug_draw,
    }
  }
}

impl Default for RayTracer {
  fn default() -> Self {
    Self {
      colliders: Vec::new(),
      debug_draw: false,
    }
  }
}

impl Component for RayTracer {
  type Storage = VecStorage<Self>;
}
