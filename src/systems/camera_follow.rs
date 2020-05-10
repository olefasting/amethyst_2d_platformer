use amethyst::{
  core::{math::Vector3, Transform},
  derive::SystemDesc,
  ecs::{Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

use crate::{components::PlayerActor, resources::ActiveCamera};

const CAMERA_MOVE_THRESHOLD: f32 = 128.0;
const CAMERA_MOVE_SPEED: f32 = 6.0;

#[derive(SystemDesc)]
pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, PlayerActor>,
    ReadExpect<'s, ActiveCamera>,
  );

  fn run(&mut self, (mut transforms, player_actors, active_camera): Self::SystemData) {
    // FIXME: Use ownership through components, in stead

    let mut camera_translation = transforms
      .get(active_camera.0)
      .expect("Entity set as active camera does not exist!")
      .translation()
      .clone();

    for (transform, _) in (&transforms, &player_actors).join() {
      let player_translation = transform.translation().clone();

      /*
      if camera_translation.x < player_translation.x - CAMERA_MOVE_THRESHOLD
        || camera_translation.x > player_translation.x + CAMERA_MOVE_THRESHOLD
      {
        // TODO: Lerp camera position
        // camera_translation.x = player_translation.x
      }
      if camera_translation.y < player_translation.y - CAMERA_MOVE_THRESHOLD
        || camera_translation.y > player_translation.y + CAMERA_MOVE_THRESHOLD
      {
        // TODO: Lerp camera position
        // camera_translation.y = player_translation.y
      }
      */

      camera_translation.x = player_translation.x;
      camera_translation.y = player_translation.y;
    }

    transforms
      .get_mut(active_camera.0)
      .expect("Entity set as active camera does not exist!")
      .set_translation(camera_translation);
  }
}
