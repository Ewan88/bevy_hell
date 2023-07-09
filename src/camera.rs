use bevy::prelude::*;

use crate::{input, player::Player};

#[derive(Component)]
pub struct GameCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, move_camera);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}

fn move_camera(
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else { return; };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return; };
    let diffx = player_transform.translation.x - camera_transform.translation.x;
    let diffy = player_transform.translation.y - camera_transform.translation.y;
    camera_transform.translation.x += diffx * time.delta_seconds() * input::SPEED;
    camera_transform.translation.y += diffy * time.delta_seconds() * input::SPEED;
}
