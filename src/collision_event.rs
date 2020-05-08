use amethyst::{ecs::world::Entity, shrev::EventChannel};

#[derive(Debug, Copy, Clone)]
pub struct CollisionEvent {
  pub source: Entity,
  pub target: Entity,
}

impl CollisionEvent {
  pub fn new(source: Entity, target: Entity) -> Self {
    Self { source, target }
  }
}

#[derive(Debug, Default)]
pub struct CollisionEventChannel(pub Option<EventChannel<CollisionEvent>>);

impl CollisionEventChannel {
  pub fn new(channel: EventChannel<CollisionEvent>) -> Self {
    Self(Some(channel))
  }
}
