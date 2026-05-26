use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::{Button, Res};
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