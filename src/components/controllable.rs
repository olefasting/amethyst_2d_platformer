use amethyst::ecs::{Component, VecStorage};

use amethyst_physics::prelude::*;

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub enum ControlAction {
  None,
  Stand,
  Idle,
  Walk,
  Run,
  Jump,
  Fall,
  Die,
  Interact,
  PrimaryAttack,
  SecondaryAttack,
}

impl Default for ControlAction {
  fn default() -> Self {
    Self::None
  }
}

impl ToString for ControlAction {
  fn to_string(&self) -> String {
    match self {
      Self::None => String::from("action_none"),
      Self::Stand => String::from("action_stand"),
      Self::Idle => String::from("action_idle"),
      Self::Walk => String::from("action_walk"),
      Self::Run => String::from("action_run"),
      Self::Jump => String::from("action_jump"),
      Self::Fall => String::from("action_fall"),
      Self::Die => String::from("action_die"),
      Self::Interact => String::from("action_interact"),
      Self::PrimaryAttack => String::from("action_primary_attack"),
      Self::SecondaryAttack => String::from("action_secondary_attack"),
    }
  }
}

impl From<&str> for ControlAction {
  fn from(s: &str) -> Self {
    match s {
      "action_stand" => Self::Stand,
      "action_idle" => Self::Idle,
      "action_walk" => Self::Walk,
      "action_run" => Self::Run,
      "action_jump" => Self::Jump,
      "action_fall" => Self::Fall,
      "action_interact" => Self::Interact,
      "action_primary_attack" => Self::PrimaryAttack,
      "action_secondary_attack" => Self::SecondaryAttack,
      _ => Self::None,
    }
  }
}

impl PartialEq for ControlAction {
  fn eq(&self, other: &ControlAction) -> bool {
    self.to_string() == other.to_string()
  }
}

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
pub struct Controllable {
  pub drag: f32,
  pub ground_acceleration: f32,
  pub ground_max_speed: f32,
  pub air_acceleration: f32,
  pub air_max_speed: f32,
  pub jump_power: f32,
  pub jump_cnt: u32,
  pub max_jump_cnt: u32,
  pub facing_right: bool,
  pub current_action: ControlAction,
  pub control_mode: ControlMode,
  contact_events: Vec<ContactEvent<f32>>,
}

impl Controllable {
  pub fn contact_events_as_ref(&mut self) -> &Vec<ContactEvent<f32>> {
    &self.contact_events
  }

  pub fn contact_events_as_mut(&mut self) -> &mut Vec<ContactEvent<f32>> {
    &mut self.contact_events
  }
}

impl Default for Controllable {
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
      current_action: ControlAction::None,
      control_mode: ControlMode::Realistic,
      contact_events: Vec::with_capacity(ACTOR_CONTACTS_TO_REPORT),
    }
  }
}

impl Component for Controllable {
  type Storage = VecStorage<Self>;
}
