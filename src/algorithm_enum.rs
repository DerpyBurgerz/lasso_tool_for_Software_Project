use bevy::input::ButtonInput;
use bevy::prelude::{Res, ResMut, Resource};
use bevy_egui::egui::Key;
use crate::algorithm_enum::PointAlgorithm::{Alg1, Alg2};

#[derive(Debug, Clone, Resource)]
pub enum PointAlgorithm {
    Alg1,
    Alg2
}

impl Default for PointAlgorithm {
    fn default() -> Self {
        Alg1
    }
}

pub fn change_algorithm_system (
    keyboard_input: Res<ButtonInput<Key>>,
    mut current_algorithm: ResMut<PointAlgorithm>,
) {
    if keyboard_input.pressed(Key::Q) {
        *current_algorithm = Alg1
    }
    if keyboard_input.pressed(Key::Q) {
        *current_algorithm = Alg2
    }

}