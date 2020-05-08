use amethyst::{
  core::math::Vector2,
  ecs::{Component, VecStorage},
};

use ncollide2d::shape::ShapeHandle;

const DEFAULT_DISTANCE_THRESHOLD_X: f32 = 128.0;
const DEFAULT_DISTANCE_THRESHOLD_Y: f32 = 128.0;

#[derive(Clone)]
pub struct Collider {
  pub shape_handle: Option<ShapeHandle<f32>>,
  pub distance_threshold: Vector2<f32>,
  pub debug_draw: bool,
}

impl Collider {
  pub fn new(
    shape_handle: ShapeHandle<f32>,
    distance_threshold: Vector2<f32>,
    debug_draw: bool,
  ) -> Self {
    Self {
      shape_handle: Some(shape_handle),
      distance_threshold,
      debug_draw,
    }
  }

  pub fn with_shape_handle(self, shape_handle: ShapeHandle<f32>) -> Self {
    Self {
      shape_handle: Some(shape_handle),
      distance_threshold: self.distance_threshold,
      debug_draw: self.debug_draw,
    }
  }

  pub fn with_distance_threshold(self, distance_threshold: Vector2<f32>) -> Self {
    Self {
      shape_handle: self.shape_handle,
      distance_threshold,
      debug_draw: self.debug_draw,
    }
  }

  pub fn with_debug_draw(self) -> Self {
    Self {
      shape_handle: self.shape_handle,
      distance_threshold: self.distance_threshold,
      debug_draw: true,
    }
  }
}

impl Default for Collider {
  fn default() -> Self {
    let distance_threshold =
      Vector2::new(DEFAULT_DISTANCE_THRESHOLD_X, DEFAULT_DISTANCE_THRESHOLD_Y);

    Self {
      shape_handle: None,
      distance_threshold,
      debug_draw: false,
    }
  }
}

impl Component for Collider {
  type Storage = VecStorage<Self>;
}
