use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component, Default, Clone)]
pub struct Shape {
    pub is_closed: bool,
    pub points: Vec<Vec2>,
}

#[derive(Component)]
pub struct IsCurrentlyBeingDrawn;

impl Shape {
    pub fn add_point(&mut self, new_point: Vec2) {
        if self.points.is_empty() {
            self.points.push(new_point);
            return;
        }

        if self.points.len() < 2 {
            if self.points.last().unwrap().distance(new_point) > 5.0 {
                self.points.push(new_point);
            }
            return;
        }

        let line_point = self.points[self.points.len() - 2];
        let between_point = self.points[self.points.len() - 1];

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

        // Distance is within threshold, between_point is redundant
        if dist <= 0.3 {
            self.points.pop();
        }

        // Only add if it's far enough from the last point to avoid excessive density
        if self.points.last().map_or(true, |p| p.distance(new_point) > 5.0) {
            self.points.push(new_point);
        }
    }
}
