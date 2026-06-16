use crate::algorithm_enum::PointAlgorithm::{Meh, Alg1, CTR};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Res, ResMut, Resource};
use crate::cumulative_triangle_routine::{cumulative_triangle_routine_step, CumulativeTriangleRoutine};
use crate::perpendicular_distance_algorithm::perpendicular_distance_algorithm;

#[derive(Debug, Clone, Resource, Default)]
pub enum PointAlgorithm {
    #[default]
    Meh,
    Alg1,
    CTR {
        ctr: CumulativeTriangleRoutine
    },
}
impl Algorithm for PointAlgorithm {
    fn simplify(self, points: &mut Vec<Vec2>) -> PointAlgorithm {
        match self {
            Meh => self,
            Alg1 => {
                perpendicular_distance_algorithm(points);
                self
            },
            CTR{ctr} => {
                CTR{
                    ctr: cumulative_triangle_routine_step(points, ctr)}
            }
        }
    }
}

pub trait Algorithm {
    fn simplify(self, points: &mut Vec<Vec2>) -> PointAlgorithm;
}
pub fn change_algorithm_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut current_algorithm: ResMut<PointAlgorithm>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        *current_algorithm = Alg1;
    }
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        *current_algorithm = Alg1;
    }
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        *current_algorithm = CTR {
            ctr: CumulativeTriangleRoutine::default()
        };
    }
}
