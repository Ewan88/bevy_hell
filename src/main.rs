mod camera;
mod enemy;
mod input;
mod loader;
mod player;

use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::BISQUE))
        .insert_resource(RapierConfiguration {
            gravity: Vector2::new(0.0, 0.0).into(),
            ..Default::default()
        })
        .add_plugin(loader::AssetLoader)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .run();
}
