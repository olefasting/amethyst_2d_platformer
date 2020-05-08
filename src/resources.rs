use amethyst::shrev::EventChannel;

use crate::{states::StateId, Collision};

#[derive(Debug, Default, Copy, Clone)]
pub struct WorldGravity(pub f32);

#[derive(Debug, Default, Clone)]
pub struct CurrentState(pub StateId);

#[derive(Debug, Default)]
pub struct Collisions(pub Vec<Collision>);
