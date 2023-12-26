use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{
    assets::Images, player::Player, random_point_within_radius, CollisionSet, GameState,
    SpawnSet,
};

#[derive(Component)]
pub struct Pickup;

#[derive(Resource)]
pub struct SpawnTimer {
    pub countdown: Timer,
}

impl SpawnTimer {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_entropy();
        Self {
            countdown: Timer::from_seconds(
                rng.gen_range(60.0..120.),
                TimerMode::Repeating,
            ),
        }
    }
}

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer::new()).add_systems(
            Update,
            (
                spawn_pickups.in_set(SpawnSet),
                pickup_collision.in_set(CollisionSet),
            ),
        );
    }
}

pub fn spawn_pickups(
    mut commands: Commands,
    icon: Res<Images>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Running {
        return;
    }
    timer.countdown.tick(time.delta());
    let Ok(&player_transform) = player_query.get_single() else {
        return;
    };
    let texture_hanlde = icon.health_potion.clone();
    if timer.countdown.finished() {
        let mut rng = SmallRng::from_entropy();

        let x_start = player_transform.translation.x;
        let y_start = player_transform.translation.y;
        let (x_offset, y_offset) =
            random_point_within_radius(&mut rng, 1280., x_start, y_start, 700.);

        let transform = Transform::from_xyz(x_start + x_offset, y_start + y_offset, 1.);
        println!("Spawning pickup at {:?}", transform.translation);
        commands.spawn((
            SpriteBundle {
                transform,
                texture: texture_hanlde.clone(),
                ..default()
            },
            Pickup,
        ));
    }
}

pub fn pickup_collision() {}
