mod animation;
mod assets;
mod attacks;
mod bullet;
mod camera;
mod debug;
mod enemy;
mod input;
mod map;
mod pickups;
mod player;
mod ui;

use std::f32::consts::PI;

use assets::Audio;
use bevy::{audio::Volume, prelude::*};
use enemy::components::Enemy;
use rand::{rngs::SmallRng, Rng};
use ui::GameOverText;

pub const SCREEN_WIDTH: f32 = 1280.;
pub const SCREEN_HEIGHT: f32 = 720.;
pub const AUDIO_VOLUME: f32 = 0.5;

pub const BASE_MOVE_SPEED: f32 = 100.;

#[derive(
    SystemSet, States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Reflect,
)]
pub enum GameState {
    #[default]
    Loading,
    Running,
    GameOver,
    Paused,
    LevelUpScreen,
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
            pickups::PickupPlugin,
            debug::DebugPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb_u8(1, 50, 45)))
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
        .add_systems(Startup, run_game)
        .add_systems(PostStartup, play_background_audio)
        .add_systems(
            Update,
            (
                listen_for_restart.run_if(in_state(GameState::GameOver)),
                listen_for_game_pause.run_if(in_state(GameState::Running)),
                listen_for_unpause.run_if(in_state(GameState::Paused)),
            ),
        )
        .add_systems(OnEnter(GameState::Running), resume_virtual_time)
        .add_systems(OnExit(GameState::Running), pause_virtual_time)
        .run();
}

fn run_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Running);
}

fn pause_virtual_time(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn resume_virtual_time(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

fn play_background_audio(mut commands: Commands, audio: Res<Audio>) {
    commands.spawn((
        AudioPlayer::<AudioSource>(audio.background_track.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::new(AUDIO_VOLUME)),
    ));
}

fn listen_for_restart(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    enemy_query: Query<(Entity, &Enemy), With<Enemy>>,
    text_query: Query<Entity, With<GameOverText>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for enemy in enemy_query.iter() {
            commands.entity(enemy.0).despawn_recursive();
        }
        for text in text_query.iter() {
            commands.entity(text).despawn_recursive();
        }
        game_state.set(GameState::Running);
    }
}

fn listen_for_game_pause(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Paused);
    }
}

fn listen_for_unpause(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Running);
    }
}

pub fn random_point_within_radius(
    rng: &mut SmallRng,
    player_x: f32,
    player_y: f32,
) -> (f32, f32) {
    let angle = rng.gen_range(0.0..PI * 2.0);
    let min = 320.;
    let radius = (SCREEN_WIDTH.powi(2) + SCREEN_HEIGHT.powi(2)).sqrt() / 2.0;
    let distance = rng.gen_range(min..radius);
    let x = player_x + distance * angle.cos();
    let y = player_y + distance * angle.sin();

    (x, y)
}
