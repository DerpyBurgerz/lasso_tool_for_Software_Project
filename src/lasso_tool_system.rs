use crate::shape::*;
use bevy::camera::{Camera, Camera2d};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use crate::algorithm_enum::{PointAlgorithm};

pub fn add_lasso_points_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut shape: Single<(&mut Shape, &IsCurrentlyBeingDrawn)>,
    algorithm: Res<PointAlgorithm>,
) {
    if mouse.pressed(MouseButton::Left)
        && let Some(pos) = window.cursor_position() {
        let (camera, camera_transform) = *camera;
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
            println!("adding point");
            let algorithm = algorithm.into_inner().clone();
            shape.0.add_point(world_pos, Some(algorithm));
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
        shape.points = second_simple_simplify(&shape.points, 20.0);
        // shape.points = simple_simplify(&shape.points, 20.0);
        commands
            .entity(entity)
            .remove::<IsCurrentlyBeingDrawn>();
    }
}

pub fn simple_simplify(input: &Vec<Vec2>, area_threshold: f64) -> Vec<Vec2> {
    let mut result = Vec::new();
    result.push(input[0]);


    let mut first = 0;
    let mut middle = 1;
    let mut last = 2;
    let mut temp_area = 0f64;
    while last < input.len() {
        // Calculate the area of the current triad
        // of points
        let area = get_area(input[first],
                            input[middle],
                            input[last]);
        // Check If it should retain points or not
        temp_area += area;
        if temp_area > area_threshold {
            temp_area = 0.0;
            result.push(input[middle]);
            result.push(input[last]);
            first = last;
            middle = first + 1;
            last = first + 2;
        }
        else {
            middle += 1;
            last += 1;
        }
    }
    result
}

pub fn second_simple_simplify(input: &Vec<Vec2>, area_threshold: f64) -> Vec<Vec2> {
    // add input[0] to result array
    let mut result = Vec::new();
    result.push(input[0]);


    let mut first = 0;
    let mut middle = 1;
    let mut last = 2;
    let mut temp_area = 0f64;
    while last < input.len() {
        // Calculate the area of the current triad
        // of points
        let area = get_area(input[first],
                            input[middle],
                            input[last]);
        // Check If it should retain points or not
        temp_area += area;
        if temp_area > area_threshold {
            temp_area = 0.0;
            result.push(input[middle]);
            //result.push(input[last]);
            first = middle;
            middle = first + 1;
            last = first + 2;
        }
        else {
            middle += 1;
            last += 1;
        }
    }
    result
}

fn get_area(p1: Vec2, p2: Vec2, p3: Vec2) -> f64 {
    let p1 = p1.as_dvec2();
    let p2 = p2.as_dvec2();
    let p3 = p3.as_dvec2();
    let (x1, y1) = (p2.x - p1.x, p2.y - p1.y);
    let (x2, y2) = (p3.x - p1.x, p3.y - p1.y);
    let cross = x1 * y2 - y1 * x2;
    cross.abs() * 0.5
}

pub fn add_new_shape_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        println!("add new shape");
        commands.spawn((Shape::default(), IsCurrentlyBeingDrawn));
    }
}