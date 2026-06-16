use crate::algorithm_enum::PointAlgorithm::{Alg1, CTR};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Res, ResMut, Resource};
use crate::cumulative_triangle_routine::CumulativeTriangleRoutine;
use crate::perpendicular_distance_algorithm::PerpendicularDistanceAlgorithm;

#[derive(Debug, Clone, Resource)]
pub enum PointAlgorithm {
    Alg1,
    CTR {
        ctr: CumulativeTriangleRoutine
    },
}
impl Algorithm for PointAlgorithm {
    fn simplify(&self, points: &mut Vec<Vec2>) {
        match self {
            Alg1 => PerpendicularDistanceAlgorithm(points),
            CTR =>todo!(),
        }
    }
}

pub trait Algorithm {
    fn simplify(&self, points: &mut Vec<Vec2>);
}

impl Default for PointAlgorithm {
    fn default() -> Self {
        Alg1
    }
}
pub fn change_algorithm_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut current_algorithm: ResMut<PointAlgorithm>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        *current_algorithm = Alg1;
    }
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        *current_algorithm = CTR {
            ctr: CumulativeTriangleRoutine::default()
        };
    }
}
