use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerActor;

impl Component for PlayerActor {
  type Storage = NullStorage<Self>;
}
