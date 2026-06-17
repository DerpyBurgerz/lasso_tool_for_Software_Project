use crate::algorithm_enum::PointAlgorithm::{CTR, Meh, MinimumDistance, PerpendicularDistance};
use crate::point_optimisation_algorithm::cumulative_triangle_routine::{
    CumulativeTriangleRoutine, cumulative_triangle_routine_step,
};
use crate::point_optimisation_algorithm::perpendicular_distance_algorithm::perpendicular_distance_algorithm;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Res, ResMut, Resource};

#[derive(Debug, Clone, Resource, Default)]
pub enum PointAlgorithm {
    #[default]
    Meh,
    PerpendicularDistance {
        perpendicular_distance: f32,
    },
    CTR {
        ctr: CumulativeTriangleRoutine,
    },
    MinimumDistance {
        minimum_distance: f32,
    },
}
impl Algorithm for PointAlgorithm {
    fn simplify(self, points: &mut Vec<Vec2>) -> PointAlgorithm {
        match self {
            Meh => self,
            PerpendicularDistance {
                perpendicular_distance,
            } => {
                perpendicular_distance_algorithm(points, perpendicular_distance);
                self
            }
            CTR { ctr } => CTR {
                ctr: cumulative_triangle_routine_step(points, ctr),
            },
            MinimumDistance { minimum_distance } => {
                // implement this function
                todo!();
                self
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
        *current_algorithm = Meh
    }
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        *current_algorithm = PerpendicularDistance {
            perpendicular_distance: 0.7,
        };
    }
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        *current_algorithm = CTR {
            ctr: CumulativeTriangleRoutine::default(),
        };
    }
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        *current_algorithm = MinimumDistance{
            minimum_distance: 1.0,
        };
    }
}
