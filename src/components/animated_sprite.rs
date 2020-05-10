use std::{collections::HashMap, time::Instant};

use amethyst::{
  assets::Handle,
  ecs::{Component, VecStorage},
  renderer::SpriteSheet,
};

use super::ControlAction;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Animation {
  pub first: usize,
  pub length: usize,
  pub current: usize,
  pub interval: Duration,
  pub loops: bool,
  pub is_active: bool,
}

impl Animation {
  pub fn new(first: usize, length: usize, interval: Duration, loops: bool) -> Self {
    Self {
      first,
      length,
      current: 0,
      interval,
      loops,
      is_active: false,
    }
  }

  pub fn activate(&mut self) {
    self.current = 0;
    self.is_active = true;
  }

  pub fn deactivate(&mut self) {
    self.current = 0;
    self.is_active = false;
  }
}

#[derive(Debug, Clone)]
pub struct AnimatedSprite {
  pub sprite_sheet_handle: Option<Handle<SpriteSheet>>,
  pub animations: HashMap<ControlAction, Animation>,
  pub current_action: ControlAction,
  pub last_change: Instant,
}

impl AnimatedSprite {
  pub fn add_animation(&mut self, action: ControlAction, animation: Animation) {
    self.animations.insert(action, animation);
  }
}

impl Default for AnimatedSprite {
  fn default() -> Self {
    Self {
      sprite_sheet_handle: None,
      animations: HashMap::new(),
      current_action: ControlAction::default(),
      last_change: Instant::now(),
    }
  }
}

impl Component for AnimatedSprite {
  type Storage = VecStorage<Self>;
}
