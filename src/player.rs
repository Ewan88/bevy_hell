use super::loader::Icons;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { health: 100 }
    }

    pub fn recieve_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!("Player health: {}", self.health)
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            setup_player.in_base_set(StartupSet::PostStartup),
        )
        .add_system(kill_player);
    }
}

fn setup_player(mut commands: Commands, icons: Res<Icons>) {
    commands.spawn((
        SpriteBundle {
            texture: icons.samurai.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        },
        Player::new(),
        RigidBody::Dynamic,
        Collider::cuboid(16., 16.),
        Restitution::coefficient(1.),
        LockedAxes::ROTATION_LOCKED,
    ));
}

fn kill_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Player)>,
) {
    let Ok((entity, player)) = player_query.get_single_mut() else { return; };
    if player.health <= 0 {
        commands.entity(entity).despawn();
    }
}
