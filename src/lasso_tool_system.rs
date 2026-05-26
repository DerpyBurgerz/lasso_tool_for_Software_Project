use crate::shape::*;
use bevy::camera::{Camera, Camera2d};
use bevy::input::ButtonInput;
use bevy::prelude::*;

pub fn add_lasso_points_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut shape: Single<(&mut Shape, &IsCurrentlyBeingDrawn)>,
) {
    if mouse.pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            let (camera, camera_transform) = *camera;
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                println!("adding point");
                shape.0.add_point(world_pos);
            }
        }
    }
}

pub fn mouse_released_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    shape: Single<(Entity, &mut Shape, &IsCurrentlyBeingDrawn)>,
) {
    if mouse.just_released(MouseButton::Left) {
        println!("shape finished");
        let (entity, mut shape, _) = shape.into_inner();
        shape.is_closed = true;
        commands
            .entity(entity)
            .remove::<IsCurrentlyBeingDrawn>();
    }
}

pub fn add_new_shape_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        println!("add new shape");
        commands.spawn((Shape::default(), IsCurrentlyBeingDrawn));
        return;
    }
}