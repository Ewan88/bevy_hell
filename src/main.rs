mod animation;
mod assets;
mod attacks;
mod bullet;
mod camera;
mod enemy;
mod grid;
mod input;
mod map;
mod pickups;
mod player;
mod ui;

use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng};

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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnSet;

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
            grid::GridPlugin,
            pickups::PickupPlugin,
        ))
        .insert_resource(ClearColor(Color::rgb_u8(1, 50, 45)))
        .configure_sets(
            Update,
            (
                SpawnSet.before(MovementSet),
                MovementSet.before(CollisionSet),
                CollisionSet.before(DespawnSet),
                DespawnSet.after(CollisionSet),
            ),
        )
        .init_state::<GameState>()
        .run();
}

pub fn random_point_within_radius(
    rng: &mut SmallRng,
    player_x: f32,
    player_y: f32,
) -> (f32, f32) {
    let angle = rng.gen_range(0.0..PI * 2.0);
    let min = (SCREEN_WIDTH.powi(2) + SCREEN_HEIGHT.powi(2)).sqrt() / 2.0;
    let radius = min * 2.0;
    let distance = rng.gen_range(min..radius);
    let x = player_x + distance * angle.cos();
    let y = player_y + distance * angle.sin();

    if (x - player_x).abs() < min && (y - player_y).abs() < min {
        return random_point_within_radius(rng, player_x, player_y);
    }

    (x, y)
}
