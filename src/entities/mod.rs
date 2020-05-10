use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  prelude::*,
  renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

pub mod camera;
pub mod level;
pub mod player;

pub use camera::*;
pub use level::*;
pub use player::*;

pub fn load_sprite_sheet(world: &mut World, name: &str, ext: &str) -> Handle<SpriteSheet> {
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
