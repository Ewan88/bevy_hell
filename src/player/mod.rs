pub mod components;
mod levelup;
pub mod systems;
use bevy::prelude::*;
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
                    kill_player.run_if(in_state(GameState::Running)),
                    damage_audio_cooldown.run_if(in_state(GameState::Running)),
                    color_change_cooldown.run_if(in_state(GameState::Running)),
                    gain_level.run_if(in_state(GameState::Running)),
                ),
            );
    }
}
