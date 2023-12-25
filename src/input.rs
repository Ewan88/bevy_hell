use crate::{camera::GameCamera, player::Player, BASE_MOVE_SPEED};
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input_system);
    }
}

const KEY_MAP: [KeyCode; 4] = [(KeyCode::W), (KeyCode::A), (KeyCode::S), (KeyCode::D)];

fn key_pressed(input: &Res<Input<KeyCode>>, key_code: KeyCode) -> bool {
    input.pressed(key_code)
}

fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Sprite), (With<Player>, Without<GameCamera>)>,
    time: Res<Time>,
) {
    let Ok((mut player_transform, mut sprite)) = player_query.get_single_mut() else {
        return;
    };
    if key_pressed(&input, KEY_MAP[0]) {
        player_transform.translation.y += 1. * BASE_MOVE_SPEED * time.delta_seconds();
    }
    if key_pressed(&input, KEY_MAP[1]) {
        player_transform.translation.x -= 1. * BASE_MOVE_SPEED * time.delta_seconds();
        sprite.flip_x = true;
    }
    if key_pressed(&input, KEY_MAP[2]) {
        player_transform.translation.y -= 1. * BASE_MOVE_SPEED * time.delta_seconds();
    }
    if key_pressed(&input, KEY_MAP[3]) {
        player_transform.translation.x += 1. * BASE_MOVE_SPEED * time.delta_seconds();
        sprite.flip_x = false;
    }
}
