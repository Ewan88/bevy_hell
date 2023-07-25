mod animation;
mod assets;
mod attacks;
mod bullet;
mod camera;
mod enemy;
mod input;
mod map;
mod player;

use bevy::prelude::*;

pub const SCREEN_WIDTH: f32 = 1280.;
pub const SCREEN_HEIGHT: f32 = 720.;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CollisionSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DespawnSet;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            assets::AssetLoader,
            camera::CameraPlugin,
            player::PlayerPlugin,
            input::InputPlugin,
            enemy::EnemyPlugin,
            map::MapPlugin,
            attacks::AttackPlugin,
            animation::AnimationPlugin,
        ))
        .configure_set(Update, MovementSet.before(CollisionSet))
        .configure_set(Update, DespawnSet.after(CollisionSet))
        .run();
}
