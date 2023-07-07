mod assets;
mod bullet;
mod camera;
mod enemy;
mod input;
mod map;
mod player;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(assets::AssetLoader)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(bullet::BulletPlugin)
        .run();
}
