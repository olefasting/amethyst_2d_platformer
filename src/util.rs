use amethyst::core::math::{Isometry3, Point3, Vector3};

use amethyst_physics::prelude::*;

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
