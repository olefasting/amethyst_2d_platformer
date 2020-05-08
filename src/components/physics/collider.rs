use amethyst::ecs::{Component, VecStorage};

use ncollide2d::shape::ShapeHandle;

#[derive(Clone)]
pub struct Collider {
  pub shape: Option<ShapeHandle<f32>>,
  pub debug_draw: bool,
}

impl Collider {
  pub fn new(shape: ShapeHandle<f32>, debug_draw: bool) -> Self {
    Self {
      shape: Some(shape),
      debug_draw,
    }
  }
}

impl Default for Collider {
  fn default() -> Self {
    Self {
      shape: None,
      debug_draw: false,
    }
  }
}

impl Component for Collider {
  type Storage = VecStorage<Self>;
}
