use std::time::Duration;

use amethyst::{
  core::{ecs::Entity, Transform},
  prelude::*,
  renderer::SpriteRender,
};

use amethyst_physics::prelude::*;

use super::load_sprite_sheet;
use crate::{components::ACTOR_CONTACTS_TO_REPORT, components::*};

pub fn create_player(world: &mut World) -> Entity {
  let mut animated_sprite = AnimatedSprite::default();
  animated_sprite
    .sprite_sheet_handle
    .replace(load_sprite_sheet(world, "player", "png"));

  let sprite_render = SpriteRender {
    sprite_sheet: animated_sprite.sprite_sheet_handle.clone().unwrap(),
    sprite_number: 0,
  };

  let mut animated_sprite = AnimatedSprite::default();
  animated_sprite.add_animation(
    ControlAction::Idle,
    Animation::new(0, 4, Duration::from_millis(300), true),
  );
  animated_sprite.add_animation(
    ControlAction::Run,
    Animation::new(4, 8, Duration::from_millis(150), true),
  );
  animated_sprite.add_animation(
    ControlAction::Walk,
    Animation::new(34, 8, Duration::from_millis(300), true),
  );
  animated_sprite.add_animation(
    ControlAction::Jump,
    Animation::new(42, 4, Duration::from_millis(50), false),
  );
  animated_sprite.add_animation(
    ControlAction::Stand,
    Animation::new(63, 1, Duration::from_millis(50), false),
  );
  animated_sprite.add_animation(
    ControlAction::Fall,
    Animation::new(63, 1, Duration::from_millis(50), false),
  );

  let mut transform = Transform::default();
  transform.set_translation_xyz(50.0, 500.0, 1.0);

  let shape_desc = ShapeDesc::Capsule {
    half_height: 16.0,
    radius: 20.0,
  };

  let collider_shape = CollisionShapeBuilder::new(shape_desc.clone()).build(world);

  let rigid_body = RigidBodyBuilder::new_dynamic_body()
    .with_friction(0.0)
    .with_own_groups(&[COLLISION_GROUP_ACTOR, COLLISION_GROUP_PLAYER])
    .with_target_groups(&[COLLISION_GROUP_GROUND, COLLISION_GROUP_ACTOR])
    .with_contacts_to_report(ACTOR_CONTACTS_TO_REPORT)
    .with_lock_rotation_xyz()
    .build(world);

  world
    .create_entity()
    .with(rigid_body)
    .with(collider_shape)
    .with(transform)
    .with(DebugShape::new(shape_desc.clone()))
    .with(animated_sprite)
    .with(sprite_render)
    .with(ControlState::default())
    .with(Controllable::default())
    .with(LocalPlayer)
    .build()
}
