use super::loader::Icons;
use super::player::Player;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems(
            (setup_spawn_timer, setup_attack_timer)
                .in_base_set(StartupSet::PostStartup),
        )
        .add_system(spawn_enemies)
        .add_system(enemy_movement)
        .add_system(enemy_attack);
    }
}

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

impl SpawnTimer {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            timer: Timer::from_seconds(
                rng.gen_range(1..5) as f32,
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
    pub timer: Timer,
}

impl AttackTimer {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
}

fn setup_attack_timer(mut commands: Commands) {
    commands.insert_resource(AttackTimer::new());
}

fn spawn_enemies(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    icon: Res<Icons>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());
    let Ok(&player_transform) = player_query.get_single() else { return; };
    let samurai_icon = icon.samurai.clone();

    if timer.timer.finished() {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-100..100) as f32;
        let y: f32 = rng.gen_range(-100..100) as f32;
        let spawns: i32 = rng.gen_range(5..10) as i32;
        commands.spawn_batch((0..spawns).map(move |_| {
            (
                SpriteBundle {
                    texture: samurai_icon.clone(),
                    transform: Transform::from_xyz(
                        player_transform.translation.x + 1280. + x,
                        player_transform.translation.y + 720. + y,
                        1.,
                    ),
                    sprite: Sprite {
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Enemy,
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
        transform.translation.x += direction.x * time.delta_seconds() * 1e2;
        transform.translation.y += direction.y * time.delta_seconds() * 1e2;
    }
}

fn enemy_attack(
    mut player_query: Query<
        (&mut Player, &Transform),
        (With<Player>, Without<Enemy>),
    >,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut timer: ResMut<AttackTimer>,
    time: Res<Time>,
) {
    let Ok((mut player_struct, player_transform)) = player_query.get_single_mut() else { return; };

    for transform in enemy_query.iter() {
        let distance = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );
        timer.timer.tick(time.delta());
        if distance.length() < 32. && timer.timer.finished() {
            player_struct.recieve_damage(34);
        }
    }
}
