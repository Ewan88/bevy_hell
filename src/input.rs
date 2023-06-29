use crate::{camera::GameCamera, enemy::Enemy, player::Player};
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input_system)
            .add_system(mouse_input_system);
    }
}

const KEY_MAP: [KeyCode; 4] =
    [(KeyCode::W), (KeyCode::A), (KeyCode::S), (KeyCode::D)];

const SPEED: f32 = 10.;

fn key_pressed(input: &Res<Input<KeyCode>>, key_code: KeyCode) -> bool {
    input.pressed(key_code)
}

fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<
        &mut Transform,
        (With<Player>, Without<GameCamera>),
    >,
    mut camera_query: Query<
        &mut Transform,
        (With<GameCamera>, Without<Player>),
    >,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else { return; };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return; };
    if key_pressed(&input, KEY_MAP[0]) {
        player_transform.translation.y += 1. * SPEED;
        camera_transform.translation.y += 1. * SPEED;
    }
    if key_pressed(&input, KEY_MAP[1]) {
        player_transform.translation.x -= 1. * SPEED;
        camera_transform.translation.x -= 1. * SPEED;
    }
    if key_pressed(&input, KEY_MAP[2]) {
        player_transform.translation.y -= 1. * SPEED;
        camera_transform.translation.y -= 1. * SPEED;
    }
    if key_pressed(&input, KEY_MAP[3]) {
        player_transform.translation.x += 1. * SPEED;
        camera_transform.translation.x += 1. * SPEED;
    }
}

fn mouse_input_system(
    input: Res<Input<MouseButton>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
) {
    let Ok(&player_transform) = player_query.get_single() else { return; };
    if input.just_pressed(MouseButton::Left) {
        for (entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < 50. {
                commands.entity(entity).despawn();
            }
        }
    }
}
