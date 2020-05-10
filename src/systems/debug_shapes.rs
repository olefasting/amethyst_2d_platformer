use amethyst::{
  core::Transform,
  derive::SystemDesc,
  ecs::{Join, ReadStorage, System, SystemData, Write},
  renderer::debug_drawing::DebugLines,
};

use crate::{components::DebugShape, util};

#[derive(Debug, Default, SystemDesc)]
pub struct DebugShapesSystem;

impl<'s> System<'s> for DebugShapesSystem {
  type SystemData = (
    ReadStorage<'s, Transform>,
    ReadStorage<'s, DebugShape>,
    Write<'s, DebugLines>,
  );

  fn run(&mut self, (transforms, debug_shapes, mut debug_lines_resource): Self::SystemData) {
    for (transform, debug_shape) in (&transforms, &debug_shapes).join() {
      // FIXME: Check if on screen before draw
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
