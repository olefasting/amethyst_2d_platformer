use amethyst::{
  core::Transform,
  ecs::{Entity, World},
  prelude::*,
  renderer::Camera,
  window::ScreenDimensions,
};

pub fn create_camera(world: &mut World) -> Entity {
  let mut transform = Transform::default();
  transform.set_translation_z(10.0);

  let (width, height) = {
    let dim = world.read_resource::<ScreenDimensions>();
    (dim.width(), dim.height())
  };

  world
    .create_entity()
    .with(transform)
    .with(Camera::standard_2d(width, height))
    .build()
}
