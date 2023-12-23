mod animation;
mod assets;
mod attacks;
mod bullet;
mod camera;
mod enemy;
mod input;
mod map;
mod player;
mod ui;

use bevy::prelude::*;

pub const SCREEN_WIDTH: f32 = 1280.;
pub const SCREEN_HEIGHT: f32 = 720.;
pub const AUDIO_VOLUME: f32 = 0.5;

pub const BASE_MOVE_SPEED: f32 = 100.;

#[derive(SystemSet, States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    GameOver,
}

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
            ui::UIPlugin,
        ))
        .insert_resource(ClearColor(Color::rgb_u8(1, 50, 45)))
        .configure_sets(Update, MovementSet.before(CollisionSet))
        .configure_sets(Update, DespawnSet.after(CollisionSet))
        .add_state::<GameState>()
        .run();
}
