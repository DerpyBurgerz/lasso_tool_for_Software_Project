use bevy::math::Vec2;

pub fn perpendicular_distance_algorithm(
    points: &mut Vec<Vec2>,
    perpendicular_distance: f32,
    last_two_points: [Vec2; 2],
) -> [Vec2; 2] {
    let last_index = points.len() - 1;

    let line_point = last_two_points[0];
    let between_point = last_two_points[1];

    let return_array = [last_two_points[1], points[last_index]];
    if points.len() < 3 {
        return return_array;
    }


    // Line segment vector from line_point to new_point
    let d = points[last_index] - line_point;
    let d2 = d.length_squared();

    // If the two points are almost the same, don't update the line yet
    if d2 < 0.0001 {
        return return_array;
    }

    // Project between_point onto the line segment [line_point, new_point]
    // t is the normalized position along the line
    let t = ((between_point - line_point).dot(d) / d2).clamp(0.0, 1.0);
    let closest_point = line_point + t * d;

    // Get distance from between_point to its projection on the line segment
    let dist = between_point.distance(closest_point);

    // Distance is within the threshold, between_point is redundant
    if dist <= perpendicular_distance {
        points[last_index - 1] = points.pop().unwrap();
    }

    return_array
}
