use std::time::Duration;

use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::{ecs::Entity, math::Vector3, Parent, Transform},
  prelude::*,
  renderer::{
    debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
    Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
  },
  window::ScreenDimensions,
};

use amethyst_physics::prelude::*;

use crate::{
  components::actor::{action::*, ACTOR_CONTACTS_TO_REPORT},
  components::*,
  resources::*,
  states::*,
  Animation,
};

const WORLD_GRAVITY: f32 = 64.0;
const WORLD_TERMINAL_VELOCITY: f32 = 300.0;

#[derive(Debug, Default)]
pub struct GameplayState;

impl SimpleState for GameplayState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    world.register::<PlayerActor>();
    world.register::<Controllable>();
    world.register::<ControlState>();
    world.register::<AnimatedSprite>();
    world.register::<DebugShape>();

    world.register::<DebugLinesComponent>();

    world.insert(DebugLines::new());
    world.insert(DebugLinesParams { line_width: 1.0 });

    world.insert(CurrentState(StateId::GameplayState));

    let gravity_vec = Vector3::new(0.0, -WORLD_GRAVITY, 0.0);
    world.insert(WorldGravity(gravity_vec));
    world.insert(WorldTerminalVelocity(Vector3::new(
      WORLD_TERMINAL_VELOCITY,
      WORLD_TERMINAL_VELOCITY,
      WORLD_TERMINAL_VELOCITY,
    )));

    setup_physics(world, &gravity_vec);
    let player = create_player(world);
    let camera = create_camera(world, player);
    world.insert(ActiveCamera(camera));

    create_ground(world);
  }

  fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    Trans::None
  }
}

fn load_sprite_sheet(world: &mut World, name: &str, ext: &str) -> Handle<SpriteSheet> {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      format!("textures/{}.{}", name, ext),
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    format!("textures/{}.ron", name),
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}

fn setup_physics(world: &mut World, _gravity_vec: &Vector3<f32>) {
  let physics_world = world.fetch::<PhysicsWorld<f32>>();
  // Not using nphysics gravity until collisions are sorted out, so
  // that player can be a kinematic body
  //
  // physics_world.world_server().set_gravity(gravity_vec);
  let gravity_vec = Vector3::new(0.0, 0.0, 0.0);
  physics_world.world_server().set_gravity(&gravity_vec);
}

fn create_camera(world: &mut World, parent: Entity) -> Entity {
  let mut transform = Transform::default();
  transform.set_translation_z(10.0);

  let (width, height) = {
    let dim = world.read_resource::<ScreenDimensions>();
    (dim.width(), dim.height())
  };

  world
    .create_entity()
    .with(transform)
    .with(Camera::standard_2d(width, height))
    .build()
}

fn create_player(world: &mut World) -> Entity {
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
    ActorAction::Idle,
    Animation::new(0, 4, Duration::from_millis(300), true),
  );
  animated_sprite.add_animation(
    ActorAction::Run,
    Animation::new(4, 8, Duration::from_millis(150), true),
  );
  animated_sprite.add_animation(
    ActorAction::Walk,
    Animation::new(34, 8, Duration::from_millis(300), true),
  );
  animated_sprite.add_animation(
    ActorAction::Jump,
    Animation::new(42, 4, Duration::from_millis(50), false),
  );
  animated_sprite.add_animation(
    ActorAction::Stand,
    Animation::new(63, 1, Duration::from_millis(50), false),
  );
  animated_sprite.add_animation(
    ActorAction::Fall,
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
    .with(PlayerActor)
    .build()
}

fn create_ground(world: &mut World) {
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
