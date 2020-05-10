use amethyst::ecs::{Component, VecStorage};

use super::actions::*;

const DEFAULT_DRAG: f32 = 10.0;

const DEFAULT_GROUND_ACCELERATION: f32 = 100.0;
const DEFAULT_GROUND_MAX_SPEED: f32 = 512.0;

const DEFAULT_AIR_ACCELERATION: f32 = 300.0;
const DEFAULT_AIR_MAX_SPEED: f32 = 512.0;

const DEFAULT_JUMP_POWER: f32 = 50.0;

#[derive(Debug, Copy, Clone)]
pub struct ActorData {
  pub drag: f32,
  pub ground_acceleration: f32,
  pub ground_max_speed: f32,
  pub air_acceleration: f32,
  pub air_max_speed: f32,
  pub jump_power: f32,
  pub jump_cnt: u32,
  pub max_jump_cnt: u32,
  pub facing_right: bool,
  pub current_action: &'static str,
}

impl Default for ActorData {
  fn default() -> Self {
    Self {
      drag: DEFAULT_DRAG,
      ground_acceleration: DEFAULT_GROUND_ACCELERATION,
      ground_max_speed: DEFAULT_GROUND_MAX_SPEED,
      air_acceleration: DEFAULT_AIR_ACCELERATION,
      air_max_speed: DEFAULT_AIR_MAX_SPEED,
      jump_power: DEFAULT_JUMP_POWER,
      jump_cnt: 0,
      max_jump_cnt: 1,
      facing_right: true,
      current_action: ACTION_IDLE,
    }
  }
}

impl Component for ActorData {
  type Storage = VecStorage<Self>;
}
