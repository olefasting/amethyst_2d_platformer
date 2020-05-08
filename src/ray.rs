use amethyst::core::math::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
  pub offset: Vector2<f32>,
  pub ray: Vector2<f32>,
}

impl Ray {
  pub fn new(offset: Vector2<f32>, ray: Vector2<f32>) -> Self {
    Self { offset, ray }
  }

  pub fn with_offset(self, offset: Vector2<f32>) -> Self {
    Self {
      offset,
      ray: self.ray,
    }
  }

  pub fn with_ray(self, ray: Vector2<f32>) -> Self {
    Self {
      offset: self.offset,
      ray,
    }
  }
}

impl Default for Ray {
  fn default() -> Self {
    Self {
      offset: Vector2::new(0.0, 0.0),
      ray: Vector2::new(1.0, 0.0),
    }
  }
}
