pub mod gameplay;

pub use gameplay::GameplayState;

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
