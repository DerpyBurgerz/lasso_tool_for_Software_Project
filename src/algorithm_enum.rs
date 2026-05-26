use bevy::input::ButtonInput;
use bevy::prelude::Res;
use bevy_egui::egui::Key;

pub enum PointAlgorithm {
    Alg1,
    Alg2
}


pub fn change_algorithm(
    keyboard_input: Res<ButtonInput<Key>>
) {
    if keyboard_input.pressed(Key::Q) {

    }

}