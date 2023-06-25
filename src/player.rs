use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player);
    }
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sam.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        },
        Player,
    ));
}
