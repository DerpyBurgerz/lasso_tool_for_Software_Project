use bevy::math::Vec2;

pub fn perpendicular_distance_algorithm(points: &mut Vec<Vec2>) {
    if points.len() < 3 {
        return;
    }
    let new_point = points.pop().unwrap();
    if points.is_empty() {
        points.push(new_point);
        return;
    }

    if points.len() < 2 {
        if points.last().unwrap().distance(new_point) > 5.0 {
            points.push(new_point);
        }
        return;
    }

    let line_point = points[points.len() - 2];
    let between_point = points[points.len() - 1];

    // Line segment vector from line_point to new_point
    let d = new_point - line_point;
    let d2 = d.length_squared();

    // If the two points are almost the same, don't update the line yet
    if d2 < 0.0001 {
        return;
    }

    // Project between_point onto the line segment [line_point, new_point]
    // t is the normalized position along the line
    let t = ((between_point - line_point).dot(d) / d2).clamp(0.0, 1.0);
    let closest_point = line_point + t * d;

    // Get distance from between_point to its projection on the line segment
    let dist = between_point.distance(closest_point);

    // Distance is within the threshold, between_point is redundant
    if dist <= 0.7 {
        points.pop();
    }

    // Only add if it's far enough from the last point to avoid excessive density
    if points.last().is_none_or(|p| p.distance(new_point) > 5.0) {
        points.push(new_point);
    }
}
