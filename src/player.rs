use crate::{assets::*, GameState};

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub recent_damage: bool,
    pub last_damage: f64,
    pub xp: u32,
    pub level: u32,
    pub next_level: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.,
            max_health: 100.,
            recent_damage: false,
            last_damage: 0.,
            xp: 0,
            level: 1,
            next_level: 1000,
        }
    }

    pub fn receive_damage(&mut self) {
        self.health -= 1.2;
        self.recent_damage = true;
    }

    pub fn xp_to_next_level(&self) -> u32 {
        2 * self.next_level
    }

    pub fn gain_xp(&mut self, xp: u32) {
        self.xp += xp;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_player).add_systems(
            Update,
            (
                kill_player,
                damage_audio_cooldown,
                color_change_cooldown,
                gain_level,
            ),
        );
    }
}

fn setup_player(mut commands: Commands, icons: Res<Images>) {
    commands.spawn((
        SpriteBundle {
            texture: icons.samurai.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        },
        Player::new(),
    ));
}

fn kill_player(
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

fn damage_audio_cooldown(
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

    let diff = player.last_damage - time.elapsed_seconds_f64();

    if diff < -0.001 {
        sound.timer.tick(time.delta());
    }

    if sound.timer.finished() {
        player.recent_damage = false;
        commands.entity(entity).despawn();
    }
}

fn color_change_cooldown(
    mut player_query: Query<(&Player, &mut Sprite), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut sprite)) = player_query.get_single_mut() else {
        return;
    };

    if !player.recent_damage {
        return;
    }

    let diff = player.last_damage - time.elapsed_seconds_f64();

    if diff < -0.1 {
        sprite.color = Color::default();
    } else {
        sprite.color = Color::RED;
    }
}

fn gain_level(mut player_query: Query<&mut Player>) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    if player.xp > player.next_level {
        player.level += 1;
        player.next_level = player.xp_to_next_level();
    }
}
