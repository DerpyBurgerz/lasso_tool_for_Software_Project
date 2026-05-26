extern crate core;
use bevy::prelude::*;
use bevy_egui::*;

mod draw_point_system;
mod lasso_tool_system;
mod shape;
mod clear_system;
mod right_panel;
mod algorithm_enum;

use crate::lasso_tool_system::*;
use crate::draw_point_system::*;
use crate::clear_system::clear_system;
use crate::right_panel::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
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
        .add_systems(EguiPrimaryContextPass, right_panel)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
