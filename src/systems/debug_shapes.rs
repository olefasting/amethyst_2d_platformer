use amethyst::{
  core::{
    math::{Isometry3, Point3, Vector3},
    SystemDesc, Time, Transform,
  },
  derive::SystemDesc,
  ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, Write},
  input::{InputHandler, StringBindings},
  renderer::{
    camera::Camera,
    debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
    palette::Srgba,
    plugins::{RenderDebugLines, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
  },
};

use amethyst_physics::prelude::*;

use crate::{
  components::DebugShape,
  resources::{ActiveCamera, ViewSize},
  util,
};

#[derive(Debug, Default, SystemDesc)]
pub struct DebugShapesSystem;

impl<'s> System<'s> for DebugShapesSystem {
  type SystemData = (
    ReadStorage<'s, Transform>,
    ReadStorage<'s, DebugShape>,
    Write<'s, DebugLines>,
    ReadExpect<'s, ActiveCamera>,
  );

  fn run(
    &mut self,
    (transforms, debug_shapes, mut debug_lines_resource, active_camera): Self::SystemData,
  ) {
    for (transform, debug_shape) in (&transforms, &debug_shapes).join() {
      // FIXME: Check if on screen before draw
      if let Some(camera_transform) = transforms.get(active_camera.0) {
        let translation = transform.translation();
        let points = util::shape_desc_to_points(&debug_shape.desc, true);

        let len = points.len();
        for i in 0..len {
          if let Some(p1) = points.get(i) {
            let ii = if i + 1 >= len { 0 } else { i + 1 };
            if let Some(p2) = points.get(ii) {
              debug_lines_resource.draw_line(
                [translation.x + p1.x, translation.y + p1.y, 1.0].into(),
                [translation.x + p2.x, translation.y + p2.y, 1.0].into(),
                debug_shape.color,
              );
            }
          }
        }
      }
    }
  }
}
