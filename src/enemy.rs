use std::f32::consts::PI;

use crate::{
    CollisionSet, DespawnSet, GameState, MovementSet, AUDIO_VOLUME, BASE_MOVE_SPEED,
};

use super::animation::{AnimationIndices, AnimationTimer};
use super::assets::*;
use super::player::*;
use bevy::audio::Volume;
use bevy::ecs::query::QueryCombinationIter;
use bevy::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub last_damage: f64,
}

impl Enemy {
    pub fn receive_damage(&mut self, damage: f32) {
        self.health -= damage;
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (setup_spawn_timer, setup_attack_timer))
            .add_systems(
                Update,
                (
                    spawn_enemies,
                    enemy_movement.in_set(MovementSet),
                    //enemy_collision.in_set(CollisionSet),
                    enemy_attack.in_set(CollisionSet),
                    color_change_cooldown,
                    despawn_enemies.in_set(DespawnSet),
                ),
            );
    }
}

#[derive(Resource)]
pub struct SpawnTimer {
    pub countdown: Timer,
}

impl SpawnTimer {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_entropy();
        Self {
            countdown: Timer::from_seconds(rng.gen_range(0.5..2.), TimerMode::Repeating),
        }
    }
}

fn setup_spawn_timer(mut commands: Commands) {
    commands.insert_resource(SpawnTimer::new());
}

#[derive(Resource)]
pub struct AttackTimer {
    pub countdown: Timer,
}

impl AttackTimer {
    pub fn new() -> Self {
        Self {
            countdown: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

fn setup_attack_timer(mut commands: Commands) {
    commands.insert_resource(AttackTimer::new());
}

fn spawn_enemies(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Enemy>,
    icon: Res<Images>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Running {
        return;
    }

    let enemy_count = enemy_query.iter().count();
    if enemy_count > 10000 {
        return;
    }

    timer.countdown.tick(time.delta());
    let Ok(&player_transform) = player_query.get_single() else {
        return;
    };
    let texture_handle = icon.blob.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 5 };

    if timer.countdown.finished() {
        let mut rng = SmallRng::from_entropy();
        let spawns: i32 = rng.gen_range(5..10);
        let x_start = player_transform.translation.x;
        let y_start = player_transform.translation.y;

        commands.spawn_batch((0..spawns).map(move |_| {
            let (x_offset, y_offset) =
                random_point_within_radius(&mut rng, 1280., x_start, y_start);
            let transform =
                Transform::from_xyz(x_start + x_offset, y_start + y_offset, 1.);

            (
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform,
                    ..Default::default()
                },
                Enemy {
                    health: 10.,
                    last_damage: 0.,
                },
                animation_indices.clone(),
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            )
        }));
    }
}

fn random_point_within_radius(
    rng: &mut SmallRng,
    radius: f32,
    player_x: f32,
    player_y: f32,
) -> (f32, f32) {
    let angle = rng.gen_range(0.0..PI * 2.0);
    let distance = rng.gen_range(270.0..radius);
    let x = player_x + distance * angle.cos();
    let y = player_y + distance * angle.sin();

    if (x - player_x).abs() < 254. && (y - player_y).abs() < 254. {
        return random_point_within_radius(rng, radius, player_x, player_y);
    }

    (x, y)
}

fn enemy_movement(
    mut enemy_query: Query<
        (&mut Transform, &mut Enemy, &mut TextureAtlasSprite),
        (With<Enemy>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (mut transform, enemy, mut sprite) in enemy_query.iter_mut() {
        let diff = enemy.last_damage - time.elapsed_seconds_f64();
        if diff > -0.5 {
            continue;
        }

        let direction = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );
        let direction = direction.normalize() * BASE_MOVE_SPEED;

        transform.translation.x += direction.x * time.delta_seconds();
        transform.translation.y += direction.y * time.delta_seconds();

        if transform.translation.x > player_transform.translation.x {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }
}

fn enemy_attack(
    mut commands: Commands,
    audio: Res<Audio>,
    mut player_query: Query<(&mut Player, &Transform), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut attack_timer: ResMut<AttackTimer>,
    audio_query: Query<&PlayerHitSound>,
    time: Res<Time>,
) {
    let Ok((mut player_struct, player_transform)) = player_query.get_single_mut() else {
        return;
    };

    for transform in enemy_query.iter() {
        let distance = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );

        attack_timer.countdown.tick(time.delta());

        if distance.length() < 32. && attack_timer.countdown.finished() {
            if audio_query.is_empty() {
                commands.spawn((
                    AudioBundle {
                        source: audio.health_down.clone(),
                        settings: PlaybackSettings::ONCE
                            .with_volume(Volume::new_relative(AUDIO_VOLUME)),
                    },
                    PlayerHitSound {
                        timer: Timer::from_seconds(5., TimerMode::Once),
                    },
                ));
            }
            player_struct.receive_damage();
            player_struct.last_damage = time.elapsed_seconds_f64();
        }
    }
}

#[allow(dead_code)]
fn enemy_collision(mut transform_query: Query<&mut Transform, With<Enemy>>) {
    let mut transforms: QueryCombinationIter<'_, '_, &mut Transform, With<Enemy>, 2> =
        transform_query.iter_combinations_mut();
    while let Some([mut transform1, mut transform2]) = transforms.fetch_next() {
        let distance = Vec2::new(
            transform1.translation.x - transform2.translation.x,
            transform1.translation.y - transform2.translation.y,
        );
        if distance.length() <= 32. {
            let direction = distance.normalize();
            transform1.translation.x += direction.x * 2.;
            transform1.translation.y += direction.y * 2.;
            transform2.translation.x -= direction.x * 2.;
            transform2.translation.y -= direction.y * 2.;
        }
    }
}

fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<(&Transform, Entity, &Enemy), With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player_transform, mut player)) = player_query.get_single_mut() else {
        return;
    };
    for (transform, entity, enemy) in enemy_query.iter() {
        let distance = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );
        if distance.length() > 4000. || distance.length() < -4000. {
            commands.entity(entity).despawn();
        } else if enemy.health <= 0. {
            let diff = enemy.last_damage - time.elapsed_seconds_f64();
            if diff < -0.5 {
                player.gain_xp(25);
                commands.entity(entity).despawn();
            }
        }
    }
}

fn color_change_cooldown(
    mut enemy_query: Query<(&Enemy, &mut TextureAtlasSprite), With<Enemy>>,
    time: Res<Time>,
) {
    for (enemy, mut sprite) in enemy_query.iter_mut() {
        let diff = enemy.last_damage - time.elapsed_seconds_f64();
        if diff > -0.2 {
            sprite.color = Color::GRAY;
        } else if diff < -0.5 {
            sprite.color = Color::default();
        }
    }
}
