use amethyst::{core::math::Vector3, ecs::Entity};

#[derive(Debug, Clone)]
pub enum StateId {
  None,
  GameplayState,
}

impl Default for StateId {
  fn default() -> StateId {
    StateId::None
  }
}

#[derive(Debug, Default, Clone)]
pub struct CurrentState(pub StateId);

#[derive(Debug, Clone)]
pub struct WorldGravity(pub Vector3<f32>);

#[derive(Debug, Clone)]
pub struct WorldTerminalVelocity(pub Vector3<f32>);
