use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Shape {
    pub is_closed: bool,
    pub points: Vec<Vec2>,
}
pub fn add_point_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut thingy: Single<&mut Shape>,
) {
    if thingy.is_closed {
        if mouse.just_pressed(MouseButton::Left) {
            thingy.points.clear();
            thingy.is_closed = false;
            return;
        }
    } else {
        let points = &mut thingy.points;
        if mouse.just_pressed(MouseButton::Left) {
            if let Some(pos) = window.cursor_position() {
                let (camera, camera_transform) = *camera;
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                    if points.len() > 2 {
                        let first_point = points[0];
                        if world_pos.distance(first_point) < 20.0 {
                            thingy.is_closed = true;
                        } else {
                            points.push(world_pos);
                        }
                    } else {
                        points.push(world_pos);
                    }
                }

            }
        }
    }
}


pub fn draw_point_system(
    mut gizmos: Gizmos,
    mut thingy: Single<&mut Shape>) {
    let is_closed = thingy.is_closed.clone();
    let points = &mut thingy.points;
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