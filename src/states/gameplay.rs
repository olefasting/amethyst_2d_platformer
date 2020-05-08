use std::time::Duration;

use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::math::Vector2,
  core::Transform,
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
  shrev::EventChannel,
};

use ncollide2d::{
  math::Vector,
  shape::{Capsule, Cuboid, ShapeHandle},
};

use crate::components::actor::actions::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::Animation;

use crate::{CollisionEvent, CollisionEventChannel};

const WORLD_GRAVITY: f32 = 6.0;

const VIEW_WIDTH: f32 = 1024.0;
const VIEW_HEIGHT: f32 = 768.0;

#[derive(Debug, Default)]
pub struct GameplayState;

impl SimpleState for GameplayState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    world.register::<ControlState>();
    world.register::<PlayerActor>();
    world.register::<ActorData>();
    world.register::<PhysicsBody>();
    world.register::<Velocity>();
    world.register::<AnimatedSprite>();
    world.register::<CameraFollow>();
    world.register::<Collider>();

    world.insert(WorldGravity(WORLD_GRAVITY));
    world.insert(CurrentState(StateId::GameplayState));
    world.insert(CollisionEventChannel::new(
      EventChannel::<CollisionEvent>::new(),
    ));

    initialize_camera(world);
    initialize_player(world);
    initialize_ground(world);

    println!("Starting game!");
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

fn initialize_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(50.0, 1500.0, 1.0);

  world
    .create_entity()
    .with(Camera::standard_2d(VIEW_WIDTH, VIEW_HEIGHT))
    .with(transform)
    .with(ActiveCamera::new(Vector2::new(VIEW_WIDTH, VIEW_HEIGHT)))
    .build();
}

fn initialize_player(world: &mut World) {
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
    ACTION_IDLE,
    Animation::new(0, 4, Duration::from_millis(300), true),
  );
  animated_sprite.add_animation(
    ACTION_RUN,
    Animation::new(4, 8, Duration::from_millis(150), true),
  );
  animated_sprite.add_animation(
    ACTION_WALK,
    Animation::new(34, 8, Duration::from_millis(300), true),
  );
  animated_sprite.add_animation(
    ACTION_JUMP,
    Animation::new(42, 4, Duration::from_millis(50), false),
  );
  animated_sprite.add_animation(
    ACTION_STAND,
    Animation::new(63, 1, Duration::from_millis(50), false),
  );

  let mut transform = Transform::default();
  transform.set_translation_xyz(50.0, 1500.0, 0.0);

  let shape_handle = ShapeHandle::new(Capsule::new(2.0, 0.25));

  world
    .create_entity()
    .with(transform)
    .with(sprite_render)
    .with(animated_sprite)
    .with(ActorData::default())
    .with(PlayerActor)
    .with(PhysicsBody::default())
    .with(Velocity::default())
    .with(ControlState::default())
    .with(Collider::new(shape_handle, false, true))
    .build();
}

fn initialize_ground(world: &mut World) {
  let sprite_render = SpriteRender {
    sprite_sheet: load_sprite_sheet(world, "ground", "png"),
    sprite_number: 0,
  };

  for i in 0..50 {
    let mut transform = Transform::default();
    transform.set_translation_xyz((i * 32) as f32, 150.0, 0.0);

    let shape_handle = ShapeHandle::new(Cuboid::new(Vector::new(1.0, 1.0)));

    world
      .create_entity()
      .with(transform)
      .with(sprite_render.clone())
      .with(Collider::new(shape_handle, true, true))
      .build();
  }
}
