use amethyst::{
  ecs::{Component, VecStorage},
  prelude::*,
  renderer::{debug_drawing::DebugLinesComponent, palette::Srgba},
};

use amethyst_physics::prelude::*;

use crate::util;

pub const COLLISION_GROUP_GROUND: u8 = 5;
pub const COLLISION_GROUP_PLAYER: u8 = 6;
pub const COLLISION_GROUP_ACTOR: u8 = 7;

pub struct RigidBodyBuilder {
  desc: RigidBodyDesc<f32>,
}

impl RigidBodyBuilder {
  pub fn new_dynamic_body() -> Self {
    let mut desc = RigidBodyDesc::default();
    desc.mode = BodyMode::Dynamic;
    Self { desc }
  }

  pub fn new_static_body() -> Self {
    let mut desc = RigidBodyDesc::default();
    desc.mode = BodyMode::Static;
    Self { desc }
  }

  pub fn new_kinematic_body() -> Self {
    let mut desc = RigidBodyDesc::default();
    desc.mode = BodyMode::Kinematic;
    Self { desc }
  }

  pub fn with_mass(mut self, mass: f32) -> Self {
    self.desc.mass = mass;
    self
  }

  pub fn with_bounciness(mut self, bounciness: f32) -> Self {
    self.desc.bounciness = bounciness;
    self
  }

  pub fn with_friction(mut self, friction: f32) -> Self {
    self.desc.friction = friction;
    self
  }

  pub fn with_own_group(mut self, own_group: u8) -> Self {
    self.desc.belong_to.push(CollisionGroup::new(own_group));
    self
  }

  pub fn with_own_groups(mut self, own_groups: &[u8]) -> Self {
    for group in own_groups {
      self.desc.belong_to.push(CollisionGroup::new(*group));
    }
    self
  }

  pub fn with_target_group(mut self, target_group: u8) -> Self {
    self
      .desc
      .collide_with
      .push(CollisionGroup::new(target_group));
    self
  }

  pub fn with_target_groups(mut self, target_groups: &[u8]) -> Self {
    for group in target_groups {
      self.desc.collide_with.push(CollisionGroup::new(*group));
    }
    self
  }

  pub fn with_mode(mut self, mode: BodyMode) -> Self {
    self.desc.mode = mode;
    self
  }

  pub fn with_lock_translation_x(mut self, lock: bool) -> Self {
    self.desc.lock_translation_x = lock;
    self
  }

  pub fn with_lock_translation_y(mut self, lock: bool) -> Self {
    self.desc.lock_translation_y = lock;
    self
  }

  pub fn with_lock_translation_z(mut self, lock: bool) -> Self {
    self.desc.lock_translation_z = lock;
    self
  }

  pub fn with_lock_translation_xyz(mut self, lock_x: bool, lock_y: bool, lock_z: bool) -> Self {
    self.desc.lock_translation_x = lock_x;
    self.desc.lock_translation_y = lock_y;
    self.desc.lock_translation_z = lock_z;
    self
  }

  pub fn with_lock_rotation_x(mut self, lock: bool) -> Self {
    self.desc.lock_rotation_x = lock;
    self
  }

  pub fn with_lock_rotation_y(mut self, lock: bool) -> Self {
    self.desc.lock_rotation_y = lock;
    self
  }

  pub fn with_lock_rotation_z(mut self, lock: bool) -> Self {
    self.desc.lock_rotation_z = lock;
    self
  }

  pub fn with_lock_rotation_xyz(mut self, lock_x: bool, lock_y: bool, lock_z: bool) -> Self {
    self.desc.lock_rotation_x = lock_x;
    self.desc.lock_rotation_y = lock_y;
    self.desc.lock_rotation_z = lock_z;
    self
  }

  pub fn with_contacts_to_report(mut self, num: usize) -> Self {
    self.desc.contacts_to_report = num;
    self
  }

  pub fn build(self, world: &mut World) -> PhysicsHandle<PhysicsRigidBodyTag> {
    let physics_world = world.fetch::<PhysicsWorld<f32>>();
    physics_world.rigid_body_server().create(&self.desc)
  }
}

impl Default for RigidBodyBuilder {
  fn default() -> Self {
    Self {
      desc: RigidBodyDesc::default(),
    }
  }
}

pub struct CollisionShapeBuilder {
  desc: ShapeDesc<f32>,
}

impl CollisionShapeBuilder {
  pub fn new(desc: ShapeDesc<f32>) -> Self {
    Self { desc }
  }

  pub fn build(self, world: &mut World) -> PhysicsHandle<PhysicsShapeTag> {
    let physics_world = world.fetch::<PhysicsWorld<f32>>();
    physics_world.shape_server().create(&self.desc)
  }
}

#[derive(Debug, Clone)]
pub struct DebugShape {
  pub desc: ShapeDesc<f32>,
  pub color: Srgba,
}

impl DebugShape {
  pub fn new(desc: ShapeDesc<f32>) -> Self {
    Self {
      desc,
      color: Srgba::new(1.0, 0.0, 0.0, 1.0),
    }
  }
}

impl Component for DebugShape {
  type Storage = VecStorage<Self>;
}

pub fn debug_lines_component(shape_desc: &ShapeDesc<f32>, color: Srgba) -> DebugLinesComponent {
  let mut debug_lines_component = DebugLinesComponent::new();

  let points = util::shape_desc_to_points(shape_desc, true);

  let len = points.len();
  for i in 0..len {
    if let Some(p1) = points.get(i) {
      let ii = if i + 1 >= len { 0 } else { i + 1 };
      if let Some(p2) = points.get(ii) {
        debug_lines_component.add_line([p1.x, p1.y, 1.0].into(), [p2.x, p2.y, 1.0].into(), color);
      }
    }
  }

  debug_lines_component
}
