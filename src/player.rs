use crate::assets::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub recent_damage: bool,
    pub last_damage: f64,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.,
            recent_damage: false,
            last_damage: 0.,
        }
    }

    pub fn receive_damage(&mut self) {
        self.health -= 1.2;
        self.recent_damage = true;
        println!("Player health: {}", self.health)
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_player).add_systems(
            Update,
            (kill_player, damage_audio_cooldown, color_change_cooldown),
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

fn kill_player(mut commands: Commands, mut player_query: Query<(Entity, &Player)>) {
    let Ok((entity, player)) = player_query.get_single_mut() else { return; };
    if player.health <= 0. {
        commands.entity(entity).despawn();
    }
}

fn damage_audio_cooldown(
    mut commands: Commands,
    mut sound_query: Query<(Entity, &mut PlayerHitSound), With<PlayerHitSound>>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    let Ok((entity, mut sound)) = sound_query.get_single_mut() else { return; };
    let Ok(mut player) = player_query.get_single_mut() else { return; };

    if !player.recent_damage {
        return;
    }

    let diff = player.last_damage - time.elapsed_seconds_f64();

    if diff < 1. {
        sound.timer.tick(time.delta());
    }

    if sound.timer.finished() {
        println!("Sound timer finished");
        player.recent_damage = false;
        commands.entity(entity).despawn();
    }
}

fn color_change_cooldown(
    mut player_query: Query<(&Player, &mut Sprite), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut sprite)) = player_query.get_single_mut() else { return; };

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
