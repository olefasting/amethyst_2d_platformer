use amethyst::{
  core::math::Vector3,
  ecs::{Component, VecStorage},
};

#[derive(Debug, Copy, Clone)]
pub struct Velocity(pub Vector3<f32>);

impl Default for Velocity {
  fn default() -> Self {
    Self(Vector3::new(0.0, 0.0, 0.0))
  }
}

impl Component for Velocity {
  type Storage = VecStorage<Self>;
}
