use amethyst::ecs::{Component, VecStorage};

use crate::Rectangle;

#[derive(Debug, Clone)]
pub struct Collider {
  pub colliders: Vec<Rectangle>,
  pub is_ground: bool,
  pub debug_draw: bool,
}

impl Collider {
  pub fn new(colliders: Vec<Rectangle>, is_ground: bool, debug_draw: bool) -> Self {
    Self {
      colliders,
      is_ground,
      debug_draw,
    }
  }
}

impl Default for Collider {
  fn default() -> Self {
    Self {
      colliders: Vec::new(),
      is_ground: false,
      debug_draw: false,
    }
  }
}

impl Component for Collider {
  type Storage = VecStorage<Self>;
}
