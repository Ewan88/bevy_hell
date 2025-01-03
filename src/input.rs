use crate::{
    attacks::Attack, camera::GameCamera, player::components::Player, GameState,
    BASE_MOVE_SPEED,
};
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.run_if(in_state(GameState::Running)));
    }
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (&mut Transform, &mut Sprite, &Player),
        (With<Player>, Without<GameCamera>),
    >,
    time: Res<Time>,
    mut attacks_query: Query<&mut Transform, (With<Attack>, Without<Player>)>,
) {
    let Ok((mut player_transform, mut sprite, player)) = player_query.get_single_mut()
    else {
        return;
    };
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        player_transform.translation.y +=
            1. * (BASE_MOVE_SPEED + player.movement_speed_mod) * time.delta_secs();
        for mut attack_transform in attacks_query.iter_mut() {
            attack_transform.translation.y +=
                1. * (BASE_MOVE_SPEED + player.movement_speed_mod) * time.delta_secs();
        }
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        player_transform.translation.x -=
            1. * (BASE_MOVE_SPEED + player.movement_speed_mod) * time.delta_secs();
        sprite.flip_x = true;
        for mut attack_transform in attacks_query.iter_mut() {
            attack_transform.translation.x -=
                1. * (BASE_MOVE_SPEED + player.movement_speed_mod) * time.delta_secs();
        }
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        player_transform.translation.y -=
            1. * (BASE_MOVE_SPEED + player.movement_speed_mod) * time.delta_secs();
        for mut attack_transform in attacks_query.iter_mut() {
            attack_transform.translation.y -=
                1. * (BASE_MOVE_SPEED + player.movement_speed_mod) * time.delta_secs();
        }
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        player_transform.translation.x +=
            1. * (BASE_MOVE_SPEED + player.movement_speed_mod) * time.delta_secs();
        sprite.flip_x = false;
        for mut attack_transform in attacks_query.iter_mut() {
            attack_transform.translation.x +=
                1. * (BASE_MOVE_SPEED + player.movement_speed_mod) * time.delta_secs();
        }
    }
}
