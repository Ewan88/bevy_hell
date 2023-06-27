mod camera;
mod enemy;
mod input;
mod loader;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BISQUE))
        .add_plugins(DefaultPlugins)
        .add_plugin(loader::AssetLoader)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .run();
}
