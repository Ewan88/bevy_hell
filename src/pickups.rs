use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{
    assets::Images, player::components::Player, random_point_within_radius, CollisionSet,
    GameState, SpawnSet,
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
                rng.gen_range(10.0..30.),
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
        let (x_offset, y_offset) = random_point_within_radius(&mut rng, x_start, y_start);

        commands.spawn((
            Sprite::from_image(texture_hanlde.clone()),
            Transform::from_xyz(x_start + x_offset, y_start + y_offset, 1.),
            Pickup,
        ));
    }
}

pub fn pickup_collision(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
    pickup_query: Query<(Entity, &Transform), With<Pickup>>,
) {
    let Ok((mut player, player_transform)) = player_query.get_single_mut() else {
        return;
    };

    for (entity, transform) in pickup_query.iter() {
        let distance = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );
        if distance.length() <= 32. {
            player.health = (player.health + 25.).min(player.max_health);
            commands.entity(entity).despawn();
        }
    }
}
