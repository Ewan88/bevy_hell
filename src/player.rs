use super::loader::Icons;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            setup_player.in_base_set(StartupSet::PostStartup),
        );
    }
}

pub fn setup_player(mut commands: Commands, icons: Res<Icons>) {
    commands.spawn((
        SpriteBundle {
            texture: icons.samurai.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        },
        Player,
    ));
}
