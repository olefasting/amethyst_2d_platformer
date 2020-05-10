use amethyst::ecs::{Component, VecStorage};

use amethyst_physics::prelude::*;

use super::actions::*;

const DEFAULT_DRAG: f32 = 10.0;

const DEFAULT_GROUND_ACCELERATION: f32 = 128.0;
const DEFAULT_GROUND_MAX_SPEED: f32 = 256.0;

const DEFAULT_AIR_ACCELERATION: f32 = 128.0;
const DEFAULT_AIR_MAX_SPEED: f32 = 256.0;

const DEFAULT_JUMP_POWER: f32 = 800.0;

pub const ACTOR_CONTACTS_TO_REPORT: usize = 512;

#[derive(Debug, Clone)]
pub enum ControlMode {
  Realistic,
  Instant,
}

#[derive(Debug, Clone)]
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
  pub control_mode: ControlMode,
  contact_events: Vec<ContactEvent<f32>>,
}

impl ActorData {
  pub fn contact_events_as_ref(&mut self) -> &Vec<ContactEvent<f32>> {
    &self.contact_events
  }

  pub fn contact_events_as_mut(&mut self) -> &mut Vec<ContactEvent<f32>> {
    &mut self.contact_events
  }
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
      control_mode: ControlMode::Realistic,
      contact_events: Vec::with_capacity(ACTOR_CONTACTS_TO_REPORT),
    }
  }
}

impl Component for ActorData {
  type Storage = VecStorage<Self>;
}
