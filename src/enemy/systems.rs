use super::components::*;
use crate::random_point_within_radius;
use crate::{
    animation::AnimationTimer, assets::*, player::components::*, AUDIO_VOLUME,
    BASE_MOVE_SPEED,
};

use bevy::audio::Volume;
use bevy::{color, prelude::*};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub fn setup_spawn_timer(mut commands: Commands) {
    commands.insert_resource(SpawnTimer::new());
}

pub fn setup_attack_timer(mut commands: Commands) {
    commands.insert_resource(AttackTimer::new());
}

pub fn spawn_enemies(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Enemy>,
    icon: Res<Images>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Ok(&player_transform) = player_query.get_single() else {
        return;
    };

    let enemy_count = enemy_query.iter().count();
    if enemy_count > 10000 {
        return;
    }

    timer.countdown.tick(time.delta());

    let texture_handle = icon.blob.clone();
    let texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    if timer.countdown.finished() {
        let mut rng = SmallRng::from_entropy();
        let elapsed_time = time.elapsed_secs();
        let new_duration = (1. - elapsed_time / 120.).max(0.1);
        let min_spawns = (elapsed_time / 60.).ceil() as i32;
        let max_spawns = (elapsed_time / 30.).ceil() as i32;
        let spawns: i32 = rng.gen_range(min_spawns..max_spawns.max(min_spawns + 1));
        let x_start = player_transform.translation.x;
        let y_start = player_transform.translation.y;

        commands.spawn_batch((0..spawns).map(move |_| {
            let (x_offset, y_offset) =
                random_point_within_radius(&mut rng, x_start, y_start);
            (
                Sprite {
                    image: texture_handle.clone(),
                    texture_atlas: Some(TextureAtlas::from(texture_atlas_handle.clone())),
                    ..default()
                },
                Transform::from_xyz(x_start + x_offset, y_start + y_offset, 1.),
                Enemy {
                    health: 10.,
                    last_damage: 0.,
                },
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            )
        }));

        timer.countdown = Timer::from_seconds(new_duration, TimerMode::Repeating);
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<
        (&mut Transform, &mut Enemy, &mut Sprite),
        (With<Enemy>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (mut transform, enemy, mut sprite) in enemy_query.iter_mut() {
        let diff = enemy.last_damage - time.elapsed_secs_f64();
        if diff > -0.5 {
            continue;
        }

        let direction = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );
        let direction = direction.normalize() * BASE_MOVE_SPEED;

        transform.translation.x += direction.x * time.delta_secs();
        transform.translation.y += direction.y * time.delta_secs();

        sprite.flip_x = transform.translation.x > player_transform.translation.x;
    }
}

pub fn enemy_attack(
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
                    AudioPlayer::<AudioSource>(audio.health_down.clone()),
                    PlaybackSettings::ONCE.with_volume(Volume::new(AUDIO_VOLUME)),
                    PlayerHitSound {
                        timer: Timer::from_seconds(5., TimerMode::Once),
                    },
                ));
            }
            player_struct.receive_damage();
            player_struct.last_damage = time.elapsed_secs_f64();
        }
    }
}

pub fn despawn_enemies(
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
            let diff = enemy.last_damage - time.elapsed_secs_f64();
            if diff < -0.5 {
                player.gain_xp(25);
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn color_change_cooldown(
    mut enemy_query: Query<(&Enemy, &mut Sprite), With<Enemy>>,
    time: Res<Time>,
) {
    for (enemy, mut sprite) in enemy_query.iter_mut() {
        let diff = enemy.last_damage - time.elapsed_secs_f64();
        if diff > -0.2 {
            sprite.color = Color::Srgba(color::palettes::basic::GRAY);
        } else if diff < -0.5 {
            sprite.color = Color::default();
        }
    }
}
