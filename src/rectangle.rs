use amethyst::core::math::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
  pub offset: Vector2<f32>,
  pub bounds: Vector2<f32>,
}

impl Rectangle {
  pub fn new(offset: Vector2<f32>, bounds: Vector2<f32>) -> Self {
    Self { offset, bounds }
  }
}

impl Default for Rectangle {
  fn default() -> Self {
    Self {
      offset: Vector2::new(0.0, 0.0),
      bounds: Vector2::new(1.0, 1.0),
    }
  }
}
