use bevy::prelude::*;

use crate::GameState;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimerOnce(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub current: usize,
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_sprites, animate_one_shots).run_if(in_state(GameState::Running)),
        );
    }
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            indices.current = if indices.current == indices.last {
                indices.first
            } else {
                indices.current + 1
            };

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = indices.current;
            }
        }
    }
}

fn animate_one_shots(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimerOnce, &mut Sprite)>,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        if indices.current == indices.last {
            continue;
        }

        timer.tick(time.delta());

        if timer.just_finished() {
            indices.current = if indices.current == indices.last {
                indices.first
            } else {
                indices.current + 1
            };

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = indices.current;
            }
        }
    }
}
