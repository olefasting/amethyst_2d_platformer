use amethyst::ecs::{Component, VecStorage};

#[derive(Debug, Copy, Clone)]
pub struct PhysicsBody {
  pub drag: f32,
  pub angular_drag: f32,
  pub immediate_start: bool,
  pub immediate_stop: bool,
  pub use_gravity: bool,
  pub is_grounded: bool,
}

impl Default for PhysicsBody {
  fn default() -> Self {
    Self {
      drag: 1.0,
      angular_drag: 0.05,
      immediate_start: false,
      immediate_stop: false,
      use_gravity: true,
      is_grounded: false,
    }
  }
}

impl Component for PhysicsBody {
  type Storage = VecStorage<Self>;
}
