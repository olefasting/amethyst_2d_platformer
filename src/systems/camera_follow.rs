use std::time::{Duration, Instant};

use amethyst::{
  core::{math::Vector3, Transform},
  derive::SystemDesc,
  ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{ActiveCamera, PlayerActor};

#[derive(SystemDesc)]
pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, ActiveCamera>,
    ReadStorage<'s, PlayerActor>,
  );

  fn run(&mut self, (mut transforms, active_cameras, player_actors): Self::SystemData) {
    let mut player_transform: Option<Transform> = None;
    for (transform, _) in (&transforms, &player_actors).join() {
      player_transform = Some(transform.clone());
      break;
    }

    match player_transform {
      Some(player_transform) => {
        for (transform, active_camera) in (&mut transforms, &active_cameras).join() {
          let player_translation = player_transform.translation();
          transform.set_translation(Vector3::new(
            player_translation.x,
            player_translation.y + (active_camera.bounds.y * 0.25),
            1.0,
          ));
          break;
        }
      }
      _ => (),
    }
  }
}
