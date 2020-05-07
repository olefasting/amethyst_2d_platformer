use amethyst::core::math::Vector3;

pub fn lerp(current: Vector3<f32>, target: Vector3<f32>, speed: f32) -> Vector3<f32> {
  Vector3::new(
    lerp_axis(current.x, target.x, speed),
    lerp_axis(current.y, target.y, speed),
    lerp_axis(current.z, target.z, speed),
  )
}

pub fn lerp_axis(current: f32, target: f32, speed: f32) -> f32 {
  if current < target {
    let new = current + speed;
    if new > target {
      target
    } else {
      new
    }
  } else {
    let new = current - speed;
    if new < target {
      target
    } else {
      new
    }
  }
}
