use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct GameCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostUpdate, move_camera);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
}

fn move_camera(
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    camera_transform.translation = camera_transform
        .translation
        .truncate()
        .lerp(player_transform.translation.truncate(), 1.)
        .extend(999.);
}
