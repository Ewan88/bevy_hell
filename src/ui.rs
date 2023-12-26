use crate::{player::*, GameState};

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerHealth;

#[derive(Component)]
pub struct XPText;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct TimeText;

#[derive(Component)]
pub struct PositionText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (build_ui, build_debugging_text)).add_systems(
            Update,
            (update_health, update_xp, update_level, update_time, update_position_text),
        );
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_text);
    }
}

fn build_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(95.0),
                height: Val::Percent(95.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section("", TextStyle::default()), TimeText));
            parent.spawn(NodeBundle {
                style: Style {
                    left: Val::Percent(0.),
                    top: Val::Percent(-5.),
                    width: Val::Percent(110.0),
                    height: Val::Percent(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0., 0., 0., 0.5).into(),
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(95.0),
                        justify_content: JustifyContent::SpaceBetween,
                        justify_items: JustifyItems::Stretch,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section("", TextStyle::default()),
                        PlayerHealth,
                    ));
                    parent.spawn((
                        TextBundle::from_section("", TextStyle::default()),
                        XPText,
                    ));
                    parent.spawn((
                        TextBundle::from_section("", TextStyle::default()),
                        LevelText,
                    ));
                });
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(110.),
                    height: Val::Percent(10.),
                    left: Val::Percent(0.),
                    bottom: Val::Percent(-5.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0., 0., 0., 0.5).into(),
                ..default()
            });
        });
}

fn build_debugging_text(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(0.),
            top: Val::Percent(7.5),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((TextBundle::from_section("", TextStyle::default()), PositionText));
    });
}

fn spawn_game_over_text(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0., 0., 0., 0.5).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font_size: 100.,
                    ..default()
                },
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

    let health = std::cmp::max(player.health as i32, 0);
    health_text.sections[0].value = format!("Health: {}", health);
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

fn update_time(time: Res<Time>, mut time_query: Query<&mut Text, With<TimeText>>) {
    let Ok(mut time_text) = time_query.get_single_mut() else {
        return;
    };

    let total_seconds = time.elapsed_seconds_f64().round() as u32;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    time_text.sections[0].value = format!("Time: {:02}:{:02}", minutes, seconds);
}

fn update_position_text(
    mut text_query: Query<&mut Text, With<PositionText>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(transform) = player_query.get_single() else {
        return;
    };

    let Ok(mut text) = text_query.get_single_mut() else {
        return;
    };

    text.sections[0].value = format!(
        "{:.2} {:.2}",
        transform.translation.x, transform.translation.y
    );
}