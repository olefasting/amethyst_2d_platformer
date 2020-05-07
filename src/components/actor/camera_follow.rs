use amethyst::ecs::{Component, Entity, VecStorage};

#[derive(Debug, Default, Copy, Clone)]
pub struct CameraFollow {
  pub camera: Option<Entity>,
}

impl Component for CameraFollow {
  type Storage = VecStorage<Self>;
}
