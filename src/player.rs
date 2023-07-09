use super::assets::Images;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { health: 100. }
    }

    pub fn recieve_damage(&mut self, damage: f32) {
        self.health -= damage;
        println!("Player health: {}", self.health)
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_player)
            .add_systems(Update, kill_player);
    }
}

fn setup_player(mut commands: Commands, icons: Res<Images>) {
    commands.spawn((
        SpriteBundle {
            texture: icons.samurai.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        },
        Player::new(),
    ));
}

fn kill_player(mut commands: Commands, mut player_query: Query<(Entity, &Player)>) {
    let Ok((entity, player)) = player_query.get_single_mut() else { return; };
    if player.health <= 0. {
        commands.entity(entity).despawn();
    }
}
