use std::{collections::HashMap, time::Instant};

use amethyst::{
  assets::Handle,
  ecs::{Component, VecStorage},
  renderer::SpriteSheet,
};

use super::{action::*, Animation};

#[derive(Debug, Clone)]
pub struct AnimatedSprite {
  pub sprite_sheet_handle: Option<Handle<SpriteSheet>>,
  pub animations: HashMap<ActorAction, Animation>,
  pub current_action: ActorAction,
  pub last_change: Instant,
}

impl AnimatedSprite {
  pub fn add_animation(&mut self, action: ActorAction, animation: Animation) {
    self.animations.insert(action, animation);
  }
}

impl Default for AnimatedSprite {
  fn default() -> Self {
    Self {
      sprite_sheet_handle: None,
      animations: HashMap::new(),
      current_action: ActorAction::default(),
      last_change: Instant::now(),
    }
  }
}

impl Component for AnimatedSprite {
  type Storage = VecStorage<Self>;
}
