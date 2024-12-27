pub mod components;
pub mod systems;
use crate::{CollisionSet, DespawnSet, GameState, MovementSet, SpawnSet};
use bevy::prelude::*;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (setup_spawn_timer, setup_attack_timer))
            .add_systems(
                Update,
                (
                    spawn_enemies
                        .in_set(SpawnSet)
                        .run_if(in_state(GameState::Running)),
                    enemy_movement
                        .in_set(MovementSet)
                        .run_if(in_state(GameState::Running)),
                    enemy_attack
                        .in_set(CollisionSet)
                        .run_if(in_state(GameState::Running)),
                    color_change_cooldown.run_if(in_state(GameState::Running)),
                    despawn_enemies
                        .in_set(DespawnSet)
                        .run_if(in_state(GameState::Running)),
                ),
            );
    }
}
