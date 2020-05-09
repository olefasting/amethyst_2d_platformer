use amethyst::{
  core::{math::Vector3, Transform},
  derive::SystemDesc,
  ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::PlayerActor;

const CAMERA_MOVE_THRESHOLD: f32 = 128.0;
const CAMERA_MOVE_SPEED: f32 = 6.0;

#[derive(SystemDesc)]
pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
  type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, PlayerActor>);

  fn run(&mut self, (mut transforms, player_actors): Self::SystemData) {
    /*
    let mut player_transform: Option<Transform> = None;
    for (transform, _) in (&transforms, &player_actors).join() {
      player_transform = Some(transform.clone());
      break;
    }

    match player_transform {
      Some(player_transform) => {
        for (transform, active_camera) in (&mut transforms).join() {
          let player_translation = player_transform.translation();
          let camera_translation = transform.translation().clone();
          transform.set_translation(Vector3::new(
            if camera_translation.x < player_translation.x - CAMERA_MOVE_THRESHOLD
              || camera_translation.x > player_translation.x + CAMERA_MOVE_THRESHOLD
            {
              lerp(
                camera_translation.x,
                player_translation.x,
                CAMERA_MOVE_SPEED,
              )
            } else {
              camera_translation.x
            },
            if camera_translation.y < player_translation.y - CAMERA_MOVE_THRESHOLD
              || camera_translation.y > player_translation.y + CAMERA_MOVE_THRESHOLD
            {
              lerp(
                camera_translation.y,
                player_translation.y + (active_camera.bounds.y * 0.25),
                CAMERA_MOVE_SPEED,
              )
            } else {
              camera_translation.y
            },
            transform.translation().z,
          ));
          break;
        }
      }
      _ => (),
    }
    */
  }
}
