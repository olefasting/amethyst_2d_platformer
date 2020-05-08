use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default, Copy, Clone)]
pub struct Ground;

impl Component for Ground {
  type Storage = NullStorage<Self>;
}
