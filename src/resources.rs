use crate::states::StateId;

#[derive(Debug, Default, Copy, Clone)]
pub struct WorldGravity(pub f32);

#[derive(Debug, Default, Clone)]
pub struct CurrentState(pub StateId);
