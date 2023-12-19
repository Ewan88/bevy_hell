use crate::player::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerHealth;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, build_ui)
            .add_systems(Update, update_ui);
    }
}

fn build_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle { ..default() })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section("", TextStyle::default()),
                PlayerHealth,
            ));
        });
}

fn update_ui(
    player_query: Query<&Player>,
    mut health_query: Query<&mut Text, With<PlayerHealth>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let Ok(mut text) = health_query.get_single_mut() else {
        return;
    };

    text.sections[0].value = format!("Health: {}", player.health);
}
