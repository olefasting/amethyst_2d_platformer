use amethyst::{
  core::{math::Vector3, Transform},
  ecs::World,
  prelude::*,
  renderer::SpriteRender,
};

use amethyst_physics::prelude::*;

use super::load_sprite_sheet;

use crate::components::physics::*;

pub fn create_level(world: &mut World) {
  let sprite_render = SpriteRender {
    sprite_sheet: load_sprite_sheet(world, "ground", "png"),
    sprite_number: 0,
  };

  let shape_desc = ShapeDesc::Cube {
    half_extents: Vector3::new(16.0, 16.0, 16.0),
  };

  for i in 0..50 {
    let mut transform = Transform::default();
    transform.set_translation_xyz((i * 32) as f32, 150.0, 0.0);

    let collider_shape = CollisionShapeBuilder::new(shape_desc.clone()).build(world);

    let rigid_body = RigidBodyBuilder::new_static_body()
      .with_friction(0.0)
      .with_own_group(COLLISION_GROUP_GROUND)
      .with_target_groups(&[COLLISION_GROUP_ACTOR, COLLISION_GROUP_PLAYER])
      .build(world);

    world
      .create_entity()
      .with(rigid_body)
      .with(collider_shape)
      .with(transform)
      .with(DebugShape::new(shape_desc.clone()))
      .with(sprite_render.clone())
      .build();
  }
}
