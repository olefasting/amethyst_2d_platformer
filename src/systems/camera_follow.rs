use amethyst::{
  core::{math::Vector3, Transform},
  derive::SystemDesc,
  ecs::{Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
  window::ScreenDimensions,
};

use amethyst_physics::PhysicsTime;

use crate::{
  components::{ActorData, PlayerActor},
  resources::ActiveCamera,
};

const HORIZONTAL_BOUNDS_SCREEN_FRACTION: f32 = 0.33;
const HORIZONTAL_BOUNDS_DIRECTION_MODIFIER: f32 = 0.25;
const HORIZONTAL_BOUNDS_MAX_PIXELS: f32 = 800.0;

const VERTICAL_BOUNDS_SCREEN_FRACTION: f32 = 0.33;
const VERTICAL_BOUNDS_MAX_PIXELS: f32 = 450.0;

const MAX_FOLLOW_SPEED: f32 = 256.0;

#[derive(SystemDesc)]
pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, ActorData>,
    ReadStorage<'s, PlayerActor>,
    ReadExpect<'s, ActiveCamera>,
    ReadExpect<'s, ScreenDimensions>,
    ReadExpect<'s, PhysicsTime>,
  );

  fn run(
    &mut self,
    (mut transforms, actor_datas, player_actors, active_camera, screen_dimensions, physics_time): Self::SystemData,
  ) {
    // TODO: LIMIT EXTENTS
    let half_horizontal_extent =
      (screen_dimensions.height() * VERTICAL_BOUNDS_SCREEN_FRACTION) / 2.0;
    let (short_horizontal_extent, long_horizontal_extent) = {
      let half_extent = (screen_dimensions.width() * HORIZONTAL_BOUNDS_SCREEN_FRACTION) / 2.0;
      (
        half_extent / HORIZONTAL_BOUNDS_DIRECTION_MODIFIER,
        half_extent + (half_extent + HORIZONTAL_BOUNDS_DIRECTION_MODIFIER),
      )
    };

    let mut camera_translation = transforms
      .get(active_camera.0)
      .expect("Transform of active camera could not be found!")
      .translation()
      .clone();

    for (transform, actor_data, _) in (&transforms, &actor_datas, &player_actors).join() {
      let player_translation = transform.translation();

      let (left_threshold, right_threshold) = if actor_data.facing_right {
        (
          camera_translation.x + long_horizontal_extent,
          camera_translation.x - short_horizontal_extent,
        )
      } else {
        (
          camera_translation.x + short_horizontal_extent,
          camera_translation.x - long_horizontal_extent,
        )
      };

      if player_translation.x <= left_threshold || player_translation.x >= right_threshold {
        let distance = (player_translation.x - camera_translation.x) * physics_time.delta_seconds();
        camera_translation.x += if distance < MAX_FOLLOW_SPEED {
          distance
        } else {
          MAX_FOLLOW_SPEED * physics_time.delta_seconds()
        };
      }

      if player_translation.y <= camera_translation.y - half_horizontal_extent
        || player_translation.y >= camera_translation.y + half_horizontal_extent
      {
        let distance = (player_translation.y - camera_translation.y) * physics_time.delta_seconds();
        camera_translation.y += if distance < MAX_FOLLOW_SPEED {
          distance
        } else {
          MAX_FOLLOW_SPEED * physics_time.delta_seconds()
        };
      }

      break;
    }

    transforms
      .get_mut(active_camera.0)
      .expect("Transform of active camera could not be found!")
      .set_translation(camera_translation);
  }
}
