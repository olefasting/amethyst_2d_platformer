use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{ControlState, PlayerActor};

#[derive(SystemDesc)]
pub struct PlayerInputSystem {
  jump_held: bool,
}

impl<'s> System<'s> for PlayerInputSystem {
  type SystemData = (
    ReadStorage<'s, PlayerActor>,
    WriteStorage<'s, ControlState>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (player_actors, mut control_states, input): Self::SystemData) {
    for (_, control_state) in (&player_actors, &mut control_states).join() {
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

      control_state.all_controls(
        input.action_is_down("up").unwrap_or(false),
        input.action_is_down("down").unwrap_or(false),
        input.action_is_down("left").unwrap_or(false),
        input.action_is_down("right").unwrap_or(false),
        jump,
      );

      break;
    }
  }
}

impl Default for PlayerInputSystem {
  fn default() -> Self {
    Self { jump_held: false }
  }
}
