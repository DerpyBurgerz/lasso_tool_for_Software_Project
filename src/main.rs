extern crate core;

use bevy::prelude::*;
use bevy_egui::*;
use crate::algorithm_enum::{change_algorithm_system, PointAlgorithm};

mod draw_point_system;
mod lasso_tool_system;
mod shape;
mod clear_system;
mod left_panel;
mod algorithm_enum;
mod mouse_simulation;
mod triangle_area;
mod point_optimisation_algorithm;

use crate::lasso_tool_system::*;
use crate::draw_point_system::*;
use crate::clear_system::clear_system;
use crate::mouse_simulation::check_s_and_mouse_position;
use crate::left_panel::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .init_resource::<PointAlgorithm>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                add_new_shape_system.before(add_lasso_points_system),
                check_s_and_mouse_position,
                add_lasso_points_system,
                mouse_released_system,
                draw_point_system,
                change_algorithm_system,
                clear_system,
            ),
        )
        .add_systems(EguiPrimaryContextPass, left_panel)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}
