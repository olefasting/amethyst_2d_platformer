use std::time::Instant;

use amethyst::{
  core::{math::Vector3, Transform},
  derive::SystemDesc,
  ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
  renderer::SpriteRender,
};

use crate::components::{actor::actions::*, ActorData, AnimatedSprite};

#[derive(SystemDesc)]
pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, ActorData>,
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, AnimatedSprite>,
  );

  fn run(
    &mut self,
    (mut transforms, actor_datas, mut sprite_renders, mut animated_sprites): Self::SystemData,
  ) {
    for (transform, actor_data, sprite_render, animated_sprite) in (
      &mut transforms,
      &actor_datas,
      &mut sprite_renders,
      &mut animated_sprites,
    )
      .join()
    {
      if actor_data.current_action != animated_sprite.current_action {
        if let Some(animation) = animated_sprite
          .animations
          .get_mut(&animated_sprite.current_action)
        {
          animation.deactivate()
        }
        animated_sprite.current_action = actor_data.current_action;
      }

      if let Some(animation) = animated_sprite
        .animations
        .get_mut(&animated_sprite.current_action)
      {
        let now = Instant::now();
        if !animation.is_active {
          animated_sprite.last_change = Instant::now();
          animation.activate();
        } else if now.duration_since(animated_sprite.last_change) >= animation.interval {
          if animation.current + 1 >= animation.length {
            if animation.loops {
              animation.current = 0;
            } else {
              animation.deactivate();
              animated_sprite.current_action = ACTION_IDLE;
            }
          } else {
            animation.current += 1;
          }

          sprite_render.sprite_number = animation.first + animation.current;
          animated_sprite.last_change = now;
        }
      }

      if (actor_data.facing_right && transform.scale().x < 0.0)
        || (!actor_data.facing_right && transform.scale().x > 0.0)
      {
        transform.set_scale(Vector3::new(
          -transform.scale().x,
          transform.scale().y,
          transform.scale().z,
        ));
      }
    }
  }
}
