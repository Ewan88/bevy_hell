pub mod components;
mod levelup;
pub mod systems;
use bevy::prelude::*;
use components::Player;
use levelup::*;
use systems::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), setup_player)
            .add_systems(
                Update,
                (
                    kill_player,
                    damage_audio_cooldown,
                    color_change_cooldown,
                    gain_level,
                    dirty_xp,
                )
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnEnter(GameState::Paused), spawn_levelup_menu)
            .add_systems(Update, menu_action.run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::Paused), despawn_menu);
    }
}

fn dirty_xp(input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Player>) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    if input.pressed(KeyCode::NumpadAdd) {
        player.xp += 100;
    }
}
