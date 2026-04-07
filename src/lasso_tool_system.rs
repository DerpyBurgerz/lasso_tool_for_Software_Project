use crate::add_point_system::Shape;
use bevy::camera::{Camera, Camera2d};
use bevy::input::ButtonInput;
use bevy::prelude::{GlobalTransform, MouseButton, Res, Single, Window, With};

pub fn add_lasso_points_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut thingy: Single<&mut Shape>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        thingy.is_closed = false;
        thingy.points.clear();

    }
    let points = &mut thingy.points;
    if mouse.pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            let (camera, camera_transform) = *camera;
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                points.push(world_pos);
            }
        }
    }
    if mouse.just_released(MouseButton::Left) {
        thingy.is_closed = true;
    }
}
