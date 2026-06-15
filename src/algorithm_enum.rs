use crate::algorithm_enum::PointAlgorithm::{Alg1, CTRSimplification};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Res, ResMut, Resource};
use crate::PerpendicularDistanceAlgorithm::PerpendicularDistanceAlgorithm;

#[derive(Debug, Clone, Resource)]
pub enum PointAlgorithm {
    Alg1,
    CTRSimplification,
}
impl Algorithm for PointAlgorithm {
    fn simplify(&self, points: &mut Vec<Vec2>) {
        match self {
            Alg1 => PerpendicularDistanceAlgorithm(points),
            CTRSimplification => todo!(),
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
        *current_algorithm = CTRSimplification;
    }
}
