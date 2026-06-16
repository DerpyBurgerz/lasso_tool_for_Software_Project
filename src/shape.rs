use bevy::math::Vec2;
use bevy::prelude::Component;
use crate::algorithm_enum::{Algorithm, PointAlgorithm};

#[derive(Component, Default, Clone)]
pub struct Shape {
    pub is_closed: bool,
    pub points: Vec<Vec2>,
    pub optimisation_algorithm: Option<PointAlgorithm>
}

#[derive(Component)]
pub struct IsCurrentlyBeingDrawn;

impl Shape {
    pub fn add_point(&mut self, new_point: Vec2, algorithm: Option<PointAlgorithm>) -> Option<PointAlgorithm> {
        self.points.push(new_point);
        if let Some(algorithm) = algorithm{
            let next_state_algorithm = algorithm.simplify(&mut self.points);
            Some(next_state_algorithm)
        } else {
            None
        }
    }
}
