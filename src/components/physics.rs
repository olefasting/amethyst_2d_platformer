use amethyst::{
  core::math::{Isometry3, Point3, Vector3},
  ecs::{Component, VecStorage},
  prelude::*,
  renderer::{debug_drawing::DebugLinesComponent, palette::Srgba},
};

use amethyst_physics::prelude::*;

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

  pub fn with_lock_translation_x(mut self) -> Self {
    self.desc.lock_translation_x = true;
    self
  }

  pub fn with_lock_translation_y(mut self) -> Self {
    self.desc.lock_translation_y = true;
    self
  }

  pub fn with_lock_translation_z(mut self) -> Self {
    self.desc.lock_translation_z = true;
    self
  }

  pub fn with_lock_translation_xyz(mut self) -> Self {
    self.desc.lock_translation_x = true;
    self.desc.lock_translation_y = true;
    self.desc.lock_translation_z = true;
    self
  }

  pub fn with_lock_rotation_x(mut self) -> Self {
    self.desc.lock_rotation_x = true;
    self
  }

  pub fn with_lock_rotation_y(mut self) -> Self {
    self.desc.lock_rotation_y = true;
    self
  }

  pub fn with_lock_rotation_z(mut self) -> Self {
    self.desc.lock_rotation_z = true;
    self
  }

  pub fn with_lock_rotation_xyz(mut self) -> Self {
    self.desc.lock_rotation_x = true;
    self.desc.lock_rotation_y = true;
    self.desc.lock_rotation_z = true;
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

  let points = shape_desc_to_points(shape_desc, true);

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

pub fn shape_desc_to_points(desc: &ShapeDesc<f32>, skip_z: bool) -> Vec<Point3<f32>> {
  match desc {
    ShapeDesc::Sphere { radius } => {
      vector_to_points(&Vector3::new(*radius, *radius, *radius), skip_z)
    }
    ShapeDesc::Cube { half_extents } => vector_to_points(&half_extents, skip_z),
    ShapeDesc::Capsule {
      half_height,
      radius,
    } => vector_to_points(
      &Vector3::new(*radius, half_height + radius, *radius),
      skip_z,
    ),
    ShapeDesc::Cylinder {
      half_height,
      radius,
    } => vector_to_points(&Vector3::new(*radius, *half_height, *radius), skip_z),
    ShapeDesc::Convex { points } | ShapeDesc::TriMesh { points, .. } => points.clone(),
    ShapeDesc::Compound { shapes } => shapes_to_points(shapes, skip_z),
    ShapeDesc::Plane => vec![],
  }
}

pub fn vector_to_points(vector: &Vector3<f32>, skip_z: bool) -> Vec<Point3<f32>> {
  let points = [
    Point3::new(-vector.x, -vector.y, 0.0),
    Point3::new(vector.x, -vector.y, 0.0),
    Point3::new(vector.x, vector.y, 0.0),
    Point3::new(-vector.x, vector.y, 0.0),
  ];
  if !skip_z {
    [
      points,
      [
        Point3::new(-vector.x, -vector.y, vector.z),
        Point3::new(vector.x, -vector.y, vector.z),
        Point3::new(vector.x, vector.y, vector.z),
        Point3::new(-vector.x, vector.y, vector.z),
      ],
    ]
    .concat()
  } else {
    points.to_vec()
  }
}

pub fn shapes_to_points(
  shapes: &Vec<(Isometry3<f32>, ShapeDesc<f32>)>,
  skip_z: bool,
) -> Vec<Point3<f32>> {
  // FIXME: Need to take rotation into account
  let mut points: Vec<Point3<f32>> = Vec::new();
  for (isometry, desc) in shapes {
    for point in shape_desc_to_points(desc, skip_z) {
      points.push(Point3::new(
        point.x + isometry.translation.x,
        point.y + isometry.translation.y,
        if skip_z {
          point.z
        } else {
          point.z + isometry.translation.z
        },
      ));
    }
  }
  points
}
