use bevy::{audio::Volume, prelude::*};

use crate::{
    assets::{Audio, Images},
    enemy::components::Enemy,
    player::components::Player,
    CollisionSet, DespawnSet, GameState, SpawnSet, AUDIO_VOLUME,
};

#[derive(Component)]
pub struct Attack {
    pub lifetime: Timer,
}

impl Attack {
    pub fn new() -> Self {
        Self {
            lifetime: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_spawn_timer).add_systems(
            Update,
            (
                spawn_attacks
                    .in_set(SpawnSet)
                    .run_if(in_state(GameState::Running)),
                attack_lifetime
                    .in_set(DespawnSet)
                    .run_if(in_state(GameState::Running)),
                attack_collision
                    .in_set(CollisionSet)
                    .run_if(in_state(GameState::Running)),
            ),
        );
    }
}

#[derive(Resource)]
pub struct AttackSpawner {
    pub cooldown: Timer,
    pub next_attack: Timer,
    pub n_attacks: u32,
    pub attack_i: u32,
}

impl AttackSpawner {
    pub fn new() -> Self {
        let mut next = Timer::from_seconds(0.5, TimerMode::Once);
        next.pause();

        Self {
            cooldown: Timer::from_seconds(2.0, TimerMode::Repeating),
            next_attack: next,
            n_attacks: 2,
            attack_i: 0,
        }
    }
}

fn setup_spawn_timer(mut commands: Commands) {
    commands.insert_resource(AttackSpawner::new());
}

fn spawn_attacks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    icon: Res<Images>,
    audio: Res<Audio>,
    mut spawner: ResMut<AttackSpawner>,
    time: Res<Time>,
) {
    spawner.cooldown.tick(time.delta());
    spawner.next_attack.tick(time.delta());
    let Ok(&player_transform) = player_query.get_single() else {
        return;
    };

    if spawner.cooldown.finished() || spawner.next_attack.finished() {
        if spawner.attack_i < spawner.n_attacks - 1 {
            spawner.next_attack.reset();
            spawner.next_attack.unpause();
            spawner.attack_i += 1;
        } else {
            spawner.next_attack.reset();
            spawner.next_attack.pause();
            spawner.attack_i = 0;
        }

        let mut x = player_transform.translation.x;
        if (spawner.attack_i % 2) == 0 {
            x -= 50.;
        } else {
            x += 50.;
        }

        commands.spawn((
            Sprite {
                image: icon.slash_attack.clone(),
                ..Default::default()
            },
            Transform::from_xyz(x, player_transform.translation.y, 0.0),
            Attack::new(),
            AudioPlayer::<AudioSource>(audio.slash_attack.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::new(AUDIO_VOLUME)),
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
    mut attack_query: Query<&Transform, (With<Attack>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Enemy, &Transform), (With<Enemy>, Without<Attack>)>,
    time: Res<Time>,
) {
    for attack_transform in attack_query.iter_mut() {
        for (mut enemy, enemy_transform) in enemy_query.iter_mut() {
            if attack_transform
                .translation
                .distance(enemy_transform.translation)
                < 50.
            {
                enemy.receive_damage(10.);
                enemy.last_damage = time.elapsed_secs_f64();
            }
        }
    }
}
