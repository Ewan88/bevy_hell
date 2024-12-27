pub mod components;
pub mod systems;
use bevy::prelude::*;
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
                ),
            );
    }
}
