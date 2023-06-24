mod camera;
mod map;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera::setup)
        .add_startup_system(map::draw_map)
        .add_startup_system(player::setup_player)
        .run();
}
