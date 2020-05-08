use amethyst::{ecs::world::Index, shrev::EventChannel};

#[derive(Debug, Copy, Clone)]
pub struct CollisionEvent {
  pub source: Index,
  pub target: Index,
}

#[derive(Debug, Default)]
pub struct CollisionEventChannel(pub Option<EventChannel<CollisionEvent>>);

impl CollisionEventChannel {
  pub fn new(channel: EventChannel<CollisionEvent>) -> Self {
    Self(Some(channel))
  }
}
