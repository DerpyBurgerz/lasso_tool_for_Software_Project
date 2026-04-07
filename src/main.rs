mod add_point_system;
mod lasso_tool_system;

use add_point_system::*;
use bevy::prelude::*;
use crate::lasso_tool_system::add_lasso_points_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (add_lasso_points_system, draw_point_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Shape::default());
}
