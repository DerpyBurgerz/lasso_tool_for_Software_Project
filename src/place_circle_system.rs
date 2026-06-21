use bevy::camera::Camera;
use bevy::input::*;
use bevy::math::Vec2;
use bevy::prelude::{Commands, GlobalTransform, KeyCode, Query, Res, ResMut, Window, With};
use bevy::window::PrimaryWindow;
use crate::algorithm_enum::PointAlgorithm;
use crate::shape::{Shape};

pub fn check_s_and_mouse_position(
    keyboard: Res<ButtonInput<KeyCode>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    point_algorithm: ResMut<PointAlgorithm>,
    mut commands: Commands,
) {
    if !keyboard.just_pressed(KeyCode::KeyS){
        return;
    }
    let Ok(window) = q_window.single() else { return };
    let Some(cursor_pos_window) = window.cursor_position() else { return };

    // Find a camera (usually the first one with a Camera component)
    let (camera, camera_transform) = match q_camera.iter().next() {
        Some(cam) => cam,
        None => return,
    };

    let Ok(cursor_pos_world) = camera.viewport_to_world_2d(camera_transform, cursor_pos_window) else {
        return;
    };

    let mut shape = Shape::default();
    let mut algorithm = Some(point_algorithm.to_owned());
    for point in circle_points(cursor_pos_world, 600f32, 800) {
        if let Some(next_state_algorithm) = shape.add_point(point, algorithm.clone()){
            algorithm = Some(next_state_algorithm);
        }
    }
    shape.is_closed = true;
    
    commands.spawn(shape);

}
fn circle_points(center: Vec2, radius: f32, num_points: usize) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(num_points);
    let step = std::f32::consts::TAU / num_points as f32;
    for i in 0..num_points {
        let angle = i as f32 * step;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Vec2::new(x, y));
    }
    points
}