use bevy::prelude::*;

use crate::{assets::Images, enemy::Enemy, player::Player};

#[derive(Component)]
pub struct Attack {
    pub lifetime: Timer,
}

impl Attack {
    pub fn new() -> Self {
        Self {
            lifetime: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_spawn_timer)
            .add_systems(Update, (spawn_attacks, attack_lifetime, attack_collision));
    }
}

#[derive(Resource)]
pub struct SpawnTimer {
    pub countdown: Timer,
}

impl SpawnTimer {
    pub fn new() -> Self {
        Self {
            countdown: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

fn setup_spawn_timer(mut commands: Commands) {
    commands.insert_resource(SpawnTimer::new());
}

fn spawn_attacks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    icon: Res<Images>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    timer.countdown.tick(time.delta());
    let Ok(player_transform) = player_query.get_single() else { return; };
    let texture_handle = icon.slash_attack.clone();
    if timer.countdown.finished() {
        commands.spawn((
            SpriteBundle {
                texture: texture_handle,
                transform: Transform::from_xyz(
                    player_transform.translation.x + 32.,
                    player_transform.translation.y,
                    0.0,
                ),
                ..Default::default()
            },
            Attack::new(),
        ));
    }
}

fn attack_lifetime(
    mut commands: Commands,
    mut attack_query: Query<(Entity, &mut Attack)>,
    time: Res<Time>,
) {
    for (entity, mut attack) in attack_query.iter_mut() {
        attack.lifetime.tick(time.delta());
        if attack.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn attack_collision(
    mut commands: Commands,
    mut attack_query: Query<&Transform, (With<Attack>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &Enemy, &Transform), (With<Enemy>, Without<Attack>)>,
) {
    for attack_transform in attack_query.iter_mut() {
        for (entity, enemy, enemy_transform) in enemy_query.iter_mut() {
            if attack_transform
                .translation
                .distance(enemy_transform.translation)
                < 50.
            {
                enemy.die(&mut commands, entity);
            }
        }
    }
}
