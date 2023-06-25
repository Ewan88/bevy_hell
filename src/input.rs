use crate::player::Player;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input_system);
    }
}

const KEY_MAP: [KeyCode; 4] = [
    (KeyCode::W), (KeyCode::A), (KeyCode::S), (KeyCode::D),
];

fn key_pressed(input: &Res<Input<KeyCode>>, key_code: KeyCode) -> bool {
    input.pressed(key_code)
}

fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = transform_query.get_single_mut() else { return; };
    if key_pressed(&input, KEY_MAP[0]) {
        transform.translation.y += 1.;
    }
    if key_pressed(&input, KEY_MAP[1]) {
        transform.translation.x -= 1.;
    }
    if key_pressed(&input, KEY_MAP[2]) {
        transform.translation.y -= 1.;
    }
    if key_pressed(&input, KEY_MAP[3]) {
        transform.translation.x += 1.;
    }
}

