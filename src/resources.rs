use amethyst::{core::math::Vector3, ecs::Entity};

use crate::states::StateId;

#[derive(Debug, Default, Clone)]
pub struct CurrentState(pub StateId);

#[derive(Debug, Clone)]
pub struct ActiveCamera(pub Entity);

#[derive(Debug, Clone)]
pub struct WorldGravity(pub Vector3<f32>);

#[derive(Debug, Clone)]
pub struct WorldTerminalVelocity(pub Vector3<f32>);
