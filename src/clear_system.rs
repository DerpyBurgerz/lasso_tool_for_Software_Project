use bevy::prelude::*;
use bevy::prelude::KeyCode::Escape;
use crate::shape::Shape;

pub fn clear_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    shapes: Query<(Entity, &Shape)>,
) {
    if keyboard_input.just_pressed(Escape) {
        for (entity, _) in shapes {
            commands.entity(entity).clear();
        }
    }
}