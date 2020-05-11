use amethyst::{
  core::Transform,
  derive::SystemDesc,
  ecs::{Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
  window::ScreenDimensions,
};

use amethyst_physics::prelude::*;

use crate::{
  components::{Controllable, LocalPlayer},
  resources::ActiveCamera,
};

const HORIZONTAL_BOUNDS_SCREEN_FRACTION: f32 = 0.33;
const HORIZONTAL_BOUNDS_DIRECTION_MODIFIER: f32 = 0.25;
const HORIZONTAL_BOUNDS_MAX_PIXELS: f32 = 800.0;

const VERTICAL_BOUNDS_SCREEN_FRACTION: f32 = 0.33;
const VERTICAL_BOUNDS_DIRECTION_MODIFIER: f32 = 0.18;
const VERTICAL_BOUNDS_MAX_PIXELS: f32 = 450.0;

const MAX_FOLLOW_SPEED: f32 = 256.0;

#[derive(SystemDesc)]
pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Controllable>,
    ReadStorage<'s, LocalPlayer>,
    ReadExpect<'s, ActiveCamera>,
    ReadExpect<'s, ScreenDimensions>,
    ReadExpect<'s, PhysicsTime>,
  );

  fn run(
    &mut self,
    (
      mut transforms,
      actor_datas,
      local_players,
      active_camera,
      screen_dimensions,
      physics_time,
    ): Self::SystemData,
  ) {
    let mut camera_translation = transforms
      .get(active_camera.0)
      .expect("Transform of active camera could not be found!")
      .translation()
      .clone();

    // TODO: LIMIT EXTENTS
    let (short_vertical_extent, long_vertical_extent) = {
      let height = screen_dimensions.height() * VERTICAL_BOUNDS_SCREEN_FRACTION;
      let half_extent = if height > VERTICAL_BOUNDS_MAX_PIXELS {
        VERTICAL_BOUNDS_MAX_PIXELS / 2.0
      } else {
        height / 2.0
      };
      (
        half_extent / VERTICAL_BOUNDS_DIRECTION_MODIFIER,
        half_extent + (half_extent * VERTICAL_BOUNDS_DIRECTION_MODIFIER),
      )
    };

    let (short_horizontal_extent, long_horizontal_extent) = {
      let width = screen_dimensions.width() * HORIZONTAL_BOUNDS_SCREEN_FRACTION;
      let half_extent = if width > HORIZONTAL_BOUNDS_MAX_PIXELS {
        HORIZONTAL_BOUNDS_MAX_PIXELS / 2.0
      } else {
        width / 2.0
      };
      (
        half_extent / HORIZONTAL_BOUNDS_DIRECTION_MODIFIER,
        half_extent + (half_extent * HORIZONTAL_BOUNDS_DIRECTION_MODIFIER),
      )
    };

    for (transform, actor_data, _) in (&transforms, &actor_datas, &local_players).join() {
      let player_translation = transform.translation();

      let (up_threshold, down_threshold) = {
        (
          camera_translation.y + long_vertical_extent,
          camera_translation.y - short_vertical_extent,
        )
      };

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

      if player_translation.y <= camera_translation.y - down_threshold
        || player_translation.y >= camera_translation.y + up_threshold
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
