use crate::{CollisionSet, MovementSet, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::animation::{AnimationIndices, AnimationTimer};
use super::assets::Images;
use super::player::Player;
use bevy::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const MOVEMENT_SPEED: f32 = 100.;

#[derive(Component)]
pub struct Collider;

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
            countdown: Timer::from_seconds(
                rng.gen_range(1..2) as f32,
                TimerMode::Repeating,
            ),
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
    icon: Res<Images>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
        let spawns: i32 = rng.gen_range(5..10) as i32;
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
            (
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform::from_xyz(pos_x, pos_y, 1.),
                    ..Default::default()
                },
                Enemy,
                Collider,
                animation_indices.clone(),
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            )
        }));
    }
}

fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else { return; };
    for (mut transform, _enemy) in enemy_query.iter_mut() {
        let direction = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );
        let direction = direction.normalize();
        transform.translation.x += direction.x * time.delta_seconds() * MOVEMENT_SPEED;
        transform.translation.y += direction.y * time.delta_seconds() * MOVEMENT_SPEED;
    }
}

fn enemy_attack(
    mut player_query: Query<(&mut Player, &Transform), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut attack_timer: ResMut<AttackTimer>,
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
            player_struct.recieve_damage(1.2);
        }
    }
}

fn enemy_collision(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Collider>)>,
    collider_query: Query<&Transform, (With<Collider>, Without<Enemy>)>,
) {
    for collider_transform in collider_query.iter() {
        for mut enemy_transform in enemy_query.iter_mut() {
            let distance = Vec2::new(
                collider_transform.translation.x - enemy_transform.translation.x,
                collider_transform.translation.y - enemy_transform.translation.y,
            );
            if distance.length() < 32. {
                enemy_transform.translation.x += distance.x;
                enemy_transform.translation.y += distance.y;
            }
        }
    }
}
