mod animation;
mod assets;
mod bullet;
mod camera;
mod enemy;
mod input;
mod map;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                ImagePlugin::default_nearest(),
            ),
            assets::AssetLoader,
            camera::CameraPlugin,
            player::PlayerPlugin,
            input::InputPlugin,
            enemy::EnemyPlugin,
            map::MapPlugin,
            bullet::BulletPlugin,
            animation::AnimationPlugin,
        ))
        .run();
}
