// TODO: Make generic
pub fn lerp(current: f32, target: f32, speed: f32) -> f32 {
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
