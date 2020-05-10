use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default, Copy, Clone)]
pub struct LocalPlayer;

impl Component for LocalPlayer {
  type Storage = NullStorage<Self>;
}
