use amethyst::{
  core::math::Vector3,
  ecs::{Component, VecStorage},
};

#[derive(Debug, Copy, Clone)]
pub struct RigidBody {
  pub mass: f32,
  pub drag: f32,
  pub angular_drag: f64,
  pub immediate_start: bool,
  pub immediate_stop: bool,
  pub use_gravity: bool,
  pub is_grounded: bool,
}

impl Default for RigidBody {
  fn default() -> Self {
    Self {
      mass: 1.0,
      drag: 1.0,
      angular_drag: 0.05,
      immediate_start: false,
      immediate_stop: false,
      use_gravity: true,
      is_grounded: false,
    }
  }
}

impl Component for RigidBody {
  type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone)]
pub struct Collider {
  offset: Vector3<f64>,
}

impl Default for Collider {
  fn default() -> Self {
    Self {
      offset: Vector3::new(0.0, 0.0, 0.0),
    }
  }
}

impl Component for Collider {
  type Storage = VecStorage<Self>;
}
