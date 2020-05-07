use std::{
  collections::HashMap,
  time::{Duration, Instant},
};

use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::Transform,
  ecs::{Component, VecStorage},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::{components::actor::actions::*, Animation};

#[derive(Debug, Clone)]
pub struct AnimatedSprite {
  pub sprite_sheet_handle: Option<Handle<SpriteSheet>>,
  pub animations: HashMap<&'static str, Animation>,
  pub current_action: &'static str,
  pub last_change: Instant,
}

impl AnimatedSprite {
  pub fn add_animation(&mut self, action: &'static str, animation: Animation) {
    self.animations.insert(action, animation);
  }
}

impl Default for AnimatedSprite {
  fn default() -> Self {
    Self {
      sprite_sheet_handle: None,
      animations: HashMap::new(),
      current_action: ACTION_IDLE,
      last_change: Instant::now(),
    }
  }
}

impl Component for AnimatedSprite {
  type Storage = VecStorage<Self>;
}
