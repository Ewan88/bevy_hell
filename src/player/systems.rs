use super::components::*;
use crate::{assets::*, GameState};
use bevy::{color, prelude::*};

pub fn setup_player(mut commands: Commands, icons: Res<Images>) {
    commands.spawn((
        Sprite::from_image(icons.samurai.clone()),
        Transform::from_xyz(0., 0., 1.),
        Player::new(),
    ));
}

pub fn kill_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Player)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let Ok((entity, player)) = player_query.get_single_mut() else {
        return;
    };
    if player.health <= 0. {
        commands.entity(entity).despawn();
        game_state.set(GameState::GameOver);
    }
}

pub fn damage_audio_cooldown(
    mut commands: Commands,
    mut sound_query: Query<(Entity, &mut PlayerHitSound), With<PlayerHitSound>>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    let Ok((entity, mut sound)) = sound_query.get_single_mut() else {
        return;
    };
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    if !player.recent_damage {
        return;
    }

    let diff = player.last_damage - time.elapsed_secs_f64();

    if diff < -0.001 {
        sound.timer.tick(time.delta());
    }

    if sound.timer.finished() {
        player.recent_damage = false;
        commands.entity(entity).despawn();
    }
}

pub fn color_change_cooldown(
    mut player_query: Query<(&Player, &mut Sprite), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut sprite)) = player_query.get_single_mut() else {
        return;
    };

    if !player.recent_damage {
        return;
    }

    let diff = player.last_damage - time.elapsed_secs_f64();

    if diff < -0.1 {
        sprite.color = Color::default();
    } else {
        sprite.color = Color::Srgba(color::palettes::basic::RED);
    }
}

pub fn gain_level(mut player_query: Query<&mut Player>) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    if player.xp > player.next_level {
        player.level += 1;
        player.next_level = player.xp_to_next_level();
    }
}
