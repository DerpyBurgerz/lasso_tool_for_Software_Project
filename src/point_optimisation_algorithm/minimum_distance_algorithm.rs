use bevy::math::Vec2;

pub fn minimum_distance_algorithm(points: &mut Vec<Vec2>, minimum_distance: f32) {
    let last_index = points.len()-1;
    if points.len() <= 2 {
        return;
    }

    if points[last_index-1].distance(points[last_index-2]) < minimum_distance{
        points[last_index-1] = points.pop().unwrap();
    }
}