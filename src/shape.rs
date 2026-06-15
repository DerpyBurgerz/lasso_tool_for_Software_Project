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
    pub fn add_point(&mut self, new_point: Vec2, algorithm: Option<impl crate::algorithm_enum::Algorithm>) {
        self.points.push(new_point);
        if let Some(algorithm) = algorithm{
            algorithm.simplify(&mut self.points);
            return;
        }
    }
}
