use amethyst::ecs::{Component, NullStorage, VecStorage};

pub mod actions;

use actions::*;

const DEFAULT_GROUND_ACCELERATION: f32 = 3.0;
const DEFAULT_GROUND_MAX_SPEED: f32 = 6.0;

const DEFAULT_AIR_ACCELERATION: f32 = 3.0;
const DEFAULT_AIR_MAX_SPEED: f32 = 6.0;

const DEFAULT_JUMP_POWER: f32 = 32.0;

#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerActor;

impl Component for PlayerActor {
  type Storage = NullStorage<Self>;
}

#[derive(Debug, Copy, Clone)]
pub struct ActorData {
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

#[derive(Debug, Default, Copy, Clone)]
pub struct ControlState {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  pub jump: bool,
}

impl ControlState {
  pub fn all_controls(&mut self, up: bool, down: bool, left: bool, right: bool, jump: bool) {
    self.up = up;
    self.down = down;
    self.left = left;
    self.right = right;
    self.jump = jump;
  }

  pub fn reset(&mut self) {
    self.up = false;
    self.down = false;
    self.left = false;
    self.right = false;
    self.jump = false;
  }
}

impl Component for ControlState {
  type Storage = VecStorage<Self>;
}
