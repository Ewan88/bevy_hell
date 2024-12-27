use bevy::prelude::*;

use crate::GameState;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites.run_if(in_state(GameState::Running)));
    }
}

fn animate_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(&mut Sprite, &mut AnimationTimer)>,
) {
    for (mut sprite, mut timer) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let Some(atlas) = &mut sprite.texture_atlas else {
                continue;
            };
            let texture_atlas = texture_atlases.get(&atlas.layout).unwrap();
            atlas.index = (atlas.index + 1) % texture_atlas.textures.len();
        }
    }
}
