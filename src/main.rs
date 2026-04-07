extern crate core;

mod add_point_system;
mod lasso_tool_system;
mod shape;
mod clear_system;

use crate::lasso_tool_system::{
    add_lasso_points_system, add_new_shape_system, mouse_released_system,
};
use add_point_system::*;
use bevy::prelude::*;
use crate::clear_system::clear_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                add_new_shape_system.before(add_lasso_points_system),
                add_lasso_points_system,
                mouse_released_system,
                draw_point_system,
                clear_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
