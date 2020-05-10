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
