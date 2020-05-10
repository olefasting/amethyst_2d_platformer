use amethyst::{
  core::math::{Vector2, Vector3},
  ecs::Entity,
};

use crate::states::StateId;

#[derive(Debug, Default, Clone)]
pub struct CurrentState(pub StateId);

#[derive(Debug, Clone)]
pub struct ActiveCamera(pub Entity);

#[derive(Debug, Clone)]
pub struct WorldGravity(pub Vector3<f32>);

#[derive(Debug, Clone)]
pub struct ViewSize(pub Vector2<f32>);

impl ViewSize {
  pub fn new(w: f32, h: f32) -> Self {
    Self(Vector2::new(w, h))
  }

  pub fn width(&self) -> f32 {
    self.0.x
  }

  pub fn height(&self) -> f32 {
    self.0.y
  }
}
