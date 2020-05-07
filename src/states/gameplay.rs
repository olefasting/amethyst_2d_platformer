use std::time::Duration;

use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::math::Vector2,
  core::Transform,
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components::actor::actions::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::Animation;

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
    world.register::<RigidBody>();
    world.register::<Velocity>();
    world.register::<AnimatedSprite>();
    world.register::<CameraFollow>();

    world.insert(WorldGravity(WORLD_GRAVITY));
    world
      .entry::<CurrentState>()
      .or_insert_with(|| CurrentState(StateId::GameplayState));

    let mut animated_sprite = AnimatedSprite::default();
    animated_sprite
      .sprite_sheet_handle
      .replace(load_sprite_sheet(world));

    initialise_camera(world);

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
      Animation::new(5, 8, Duration::from_millis(150), true),
    );
    animated_sprite.add_animation(
      ACTION_WALK,
      Animation::new(35, 8, Duration::from_millis(300), true),
    );
    animated_sprite.add_animation(
      ACTION_JUMP,
      Animation::new(43, 4, Duration::from_millis(50), false),
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(50.0, 300.0, 0.0);

    world
      .create_entity()
      .with(transform)
      .with(sprite_render)
      .with(animated_sprite)
      .with(ActorData::default())
      .with(PlayerActor)
      .with(RigidBody::default())
      .with(Velocity::default())
      .with(ControlState::default())
      .build();

    println!("Starting game!");
  }

  fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    Trans::None
  }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "textures/spritesheet.png",
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "textures/spritesheet.ron",
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}

fn initialise_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(VIEW_WIDTH * 0.5, VIEW_HEIGHT * 0.5, 1.0);

  world
    .create_entity()
    .with(Camera::standard_2d(VIEW_WIDTH, VIEW_HEIGHT))
    .with(transform)
    .with(ActiveCamera::new(Vector2::new(VIEW_WIDTH, VIEW_HEIGHT)))
    .build();
}
