use super::loader::Icons;
use super::player::Player;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            setup_enemy_spawning.in_base_set(StartupSet::PostStartup),
        )
        .add_system(spawn_enemies.after(setup_enemy_spawning));
    }
}

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

impl SpawnTimer {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
        }
    }
}

fn setup_enemy_spawning(mut commands: Commands) {
    commands.insert_resource(SpawnTimer::new());
}

fn spawn_enemies(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    icon: Res<Icons>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());
    let Ok(player_transform) = player_query.get_single() else { return; };
    if timer.timer.finished() {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-100..100) as f32;
        let y: f32 = rng.gen_range(-100..100) as f32;
        commands.spawn((
            SpriteBundle {
                texture: icon.samurai.clone(),
                transform: Transform::from_xyz(
                    player_transform.translation.x + x,
                    player_transform.translation.y + y,
                    1.,
                ),
                sprite: Sprite {
                    color: Color::BLACK,
                    ..Default::default()
                },
                ..Default::default()
            },
            Enemy,
        ));
    }
}
