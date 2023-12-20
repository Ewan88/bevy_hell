use crate::player::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerHealth;

#[derive(Component)]
pub struct XPText;

#[derive(Component)]
pub struct LevelText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, build_ui)
            .add_systems(Update, (update_health, update_xp, update_level));
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
            parent.spawn((TextBundle::from_section("", TextStyle::default()), XPText));
            parent.spawn((
                TextBundle::from_section("", TextStyle::default()),
                LevelText,
            ));
        });
}

fn update_health(
    player_query: Query<&Player>,
    mut health_query: Query<&mut Text, With<PlayerHealth>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let Ok(mut health_text) = health_query.get_single_mut() else {
        return;
    };

    health_text.sections[0].value = format!("Health: {}", player.health.round());
}

fn update_xp(player_query: Query<&Player>, mut xp_query: Query<&mut Text, With<XPText>>) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let Ok(mut xp_text) = xp_query.get_single_mut() else {
        return;
    };

    xp_text.sections[0].value = format!("XP: {} / {}", player.xp, player.next_level);
}

fn update_level(
    player_query: Query<&Player>,
    mut level_query: Query<&mut Text, With<LevelText>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let Ok(mut level_text) = level_query.get_single_mut() else {
        return;
    };

    level_text.sections[0].value = format!("Level: {}", player.level);
}
