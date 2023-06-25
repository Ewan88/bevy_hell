mod camera;
mod map;
mod player;
mod input;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(input::InputPlugin)
        .run();
}
