use amethyst::{
  core::math::Vector2,
  ecs::{Component, VecStorage},
};

#[derive(Debug, Copy, Clone)]
pub struct ActiveCamera {
  pub bounds: Vector2<f32>,
}

impl ActiveCamera {
  pub fn new(bounds: Vector2<f32>) -> Self {
    Self { bounds }
  }
}

impl Default for ActiveCamera {
  fn default() -> Self {
    Self {
      bounds: Vector2::new(0.0, 0.0),
    }
  }
}

impl Component for ActiveCamera {
  type Storage = VecStorage<Self>;
}
