use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{ControlState, PlayerActor};

#[derive(SystemDesc)]
pub struct LocalPlayerSystem {
  jump_held: bool,
}

impl<'s> System<'s> for LocalPlayerSystem {
  type SystemData = (
    ReadStorage<'s, PlayerActor>,
    WriteStorage<'s, ControlState>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (player_actors, mut control_states, input): Self::SystemData) {
    for (_, control_state) in (&player_actors, &mut control_states).join() {
      control_state.clear();

      let jump = if input.action_is_down("jump").unwrap_or(false) {
        if self.jump_held {
          false
        } else {
          self.jump_held = true;
          true
        }
      } else {
        self.jump_held = false;
        false
      };

      let up = input.action_is_down("up").unwrap_or(false);
      let down = input.action_is_down("down").unwrap_or(false);
      let left = input.action_is_down("left").unwrap_or(false);
      let right = input.action_is_down("right").unwrap_or(false);

      control_state.set_all(
        up && !down,
        down && !up,
        left && !right,
        right && !left,
        jump,
      );

      break;
    }
  }
}

impl Default for LocalPlayerSystem {
  fn default() -> Self {
    Self { jump_held: false }
  }
}
