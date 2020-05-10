#[derive(Debug, Copy, Clone, Hash, Eq)]
pub enum ActorAction {
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

impl Default for ActorAction {
  fn default() -> Self {
    Self::None
  }
}

impl ToString for ActorAction {
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

impl From<&str> for ActorAction {
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

impl PartialEq for ActorAction {
  fn eq(&self, other: &ActorAction) -> bool {
    self.to_string() == other.to_string()
  }
}
