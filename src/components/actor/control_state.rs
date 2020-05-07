use amethyst::ecs::{Component, VecStorage};

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
