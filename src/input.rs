use crate::{bullet::Bullet, camera::GameCamera, player::Player};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else { return; };
    if key_pressed(&input, KEY_MAP[0]) {
        player_transform.translation.y += 1. * SPEED;
    }
    if key_pressed(&input, KEY_MAP[1]) {
        player_transform.translation.x -= 1. * SPEED;
    }
    if key_pressed(&input, KEY_MAP[2]) {
        player_transform.translation.y -= 1. * SPEED;
    }
    if key_pressed(&input, KEY_MAP[3]) {
        player_transform.translation.x += 1. * SPEED;
    }
}

fn mouse_input_system(
    input: Res<Input<MouseButton>>,
    window: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    let Ok(window) = window.get_single() else { return; };
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };
    let Ok(&player_transform) = player_query.get_single() else { return; };
    if input.pressed(MouseButton::Left) {
        let Some(cursor_position) = window.cursor_position() else { return; };
        let Some(direction) = camera.viewport_to_world_2d(
            camera_transform,
            cursor_position
        ) else { return; };
        let position = player_transform.translation + Vec3::new(32., 0., 0.);
        let diff = direction - position.truncate();
        commands.spawn((
            RigidBody::Dynamic,
            TransformBundle::from(Transform::from_translation(position)),
            Collider::ball(2.),
            Bullet::new(diff.normalize()),
        ));
    }
}
