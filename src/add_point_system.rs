use bevy::prelude::*;
use crate::shape::*;

pub fn draw_point_system(
    mut gizmos: Gizmos,
    shapes: Query<&mut Shape>) {

    for mut shape in shapes {
        let is_closed = shape.is_closed.clone();
        let points = &mut shape.points;
        if points.len() > 1 {
            for i in 0..points.len() - 1 {
                gizmos.line_2d(points[i], points[i + 1], Color::WHITE);
            }
            if is_closed {
                gizmos.line_2d(points[points.len() - 1], points[0], Color::WHITE);
            }
        }

        for point in points.iter() {
            gizmos.circle_2d(*point, 5.0, Color::srgb(1.0, 0.0, 0.0));
        }
    }
}