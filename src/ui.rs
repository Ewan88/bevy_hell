use crate::player::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct UI;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, build_ui);
    }
}

fn build_ui(mut commands: Commands) {
    commands
        .spawn((NodeBundle { ..default() }, UI))
        .with_children(|parent| {
            parent.spawn(TextBundle::default());
        });
}
