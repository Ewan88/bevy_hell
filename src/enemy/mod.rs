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
                    spawn_enemies.in_set(SpawnSet),
                    enemy_movement.in_set(MovementSet),
                    enemy_attack.in_set(CollisionSet),
                    //color_change_cooldown,
                    despawn_enemies.in_set(DespawnSet),
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}
