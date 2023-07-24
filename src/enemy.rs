use crate::{CollisionSet, MovementSet, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::animation::{AnimationIndices, AnimationTimer};
use super::assets::*;
use super::player::*;
use bevy::audio::Volume;
use bevy::ecs::query::QueryCombinationIter;
use bevy::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const MOVEMENT_SPEED: f32 = 100.;

#[derive(Component)]
pub struct Enemy;

impl Enemy {
    pub fn die(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).despawn();
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, MovementSet.before(CollisionSet))
            .add_systems(PostStartup, (setup_spawn_timer, setup_attack_timer))
            .add_systems(
                Update,
                (
                    spawn_enemies,
                    enemy_movement.in_set(MovementSet),
                    enemy_collision.in_set(CollisionSet),
                    enemy_attack.in_set(CollisionSet),
                    despawn_enemies,
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
) {
    let enemy_count = enemy_query.iter().count();
    if enemy_count > 1000 {
        return;
    }
    timer.countdown.tick(time.delta());
    let Ok(&player_transform) = player_query.get_single() else { return; };
    let texture_handle = icon.blob.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 5 };
    if timer.countdown.finished() {
        let mut rng = SmallRng::from_entropy();
        let x: f32 = rng.gen_range(-100..100) as f32;
        let y: f32 = rng.gen_range(-100..100) as f32;
        let spawns: i32 = rng.gen_range(5..12);
        commands.spawn_batch((0..spawns).map(move |pos| {
            let mut pos_x = player_transform.translation.x + x + (pos as f32 * 32.);
            if pos_x < 0. {
                pos_x += -SCREEN_WIDTH;
            } else {
                pos_x += SCREEN_WIDTH;
            }
            let mut pos_y =
                player_transform.translation.y + y + rng.gen_range(-60..60) as f32;
            if pos_y < 0. {
                pos_y += -SCREEN_HEIGHT;
            } else {
                pos_y += SCREEN_HEIGHT;
            }
            let transform = Transform::from_xyz(pos_x, pos_y, 1.);
            // println!("{} {}", pos_x, pos_y);
            (
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform,
                    ..Default::default()
                },
                Enemy,
                animation_indices.clone(),
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            )
        }));
    }
}

fn enemy_movement(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else { return; };
    for mut transform in enemy_query.iter_mut() {
        let direction = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );
        let direction = direction.normalize();
        transform.translation.x += direction.x * time.delta_seconds() * MOVEMENT_SPEED;
        transform.translation.y += direction.y * time.delta_seconds() * MOVEMENT_SPEED;
    }
}

#[allow(clippy::type_complexity)]
fn enemy_attack(
    mut commands: Commands,
    audio: Res<Audio>,
    mut player_query: Query<(&mut Player, &Transform), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut attack_timer: ResMut<AttackTimer>,
    audio_query: Query<&PlayerHitSound>,
    time: Res<Time>,
) {
    let Ok((mut player_struct, player_transform)) = player_query.get_single_mut() else { return; };

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
                            .with_volume(Volume::new_relative(1.)),
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
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return; };
    for (transform, entity) in enemy_query.iter() {
        let distance = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );
        if distance.length() > 4000. || distance.length() < -4000. {
            println!("Despawning enemy {:?} at {:?}", entity, transform);
            commands.entity(entity).despawn();
        }
    }
}
