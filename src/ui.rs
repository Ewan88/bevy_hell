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

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, build_ui)
            .add_systems(
                Update,
                (
                    update_health,
                    update_xp,
                    update_level,
                    update_time,
                ),
            );
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_text);
    }
}

fn build_ui(mut commands: Commands) {
    commands
        .spawn(Node {
                width: Val::Percent(95.0),
                height: Val::Percent(95.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            }
        )
        .with_children(|parent| {
            parent.spawn((Text::new(""), TimeText));
            parent.spawn((Node {
                    left: Val::Percent(0.),
                    top: Val::Percent(-5.),
                    width: Val::Percent(110.0),
                    height: Val::Percent(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor::from(Color::srgba(0., 0., 0., 0.5)),
            ));
            parent
                .spawn(Node {
                        width: Val::Percent(95.0),
                        justify_content: JustifyContent::SpaceBetween,
                        justify_items: JustifyItems::Stretch,
                        align_self: AlignSelf::Center,
                        ..default()
                    }
                )
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(""),
                        PlayerHealth,
                    ));
                    parent.spawn((
                        Text::new(""),
                        XPText,
                    ));
                    parent.spawn((
                        Text::new(""),
                        LevelText,
                    ));
                });
            parent.spawn((Node {
                    width: Val::Percent(110.),
                    height: Val::Percent(10.),
                    left: Val::Percent(0.),
                    bottom: Val::Percent(-5.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor::from(Color::srgba(0., 0., 0., 0.5)),
            ));
        });
    }

fn spawn_game_over_text(mut commands: Commands) {
    commands
        .spawn((Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor::from(Color::srgba(0., 0. , 0., 0.5))
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::new("GAME OVER"),
                TextFont {
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
    **health_text = format!("Health: {}", health);
}

fn update_xp(player_query: Query<&Player>, mut xp_query: Query<&mut Text, With<XPText>>) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let Ok(mut xp_text) = xp_query.get_single_mut() else {
        return;
    };

    **xp_text = format!("XP: {} / {}", player.xp, player.next_level);
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

    **level_text = format!("Level: {}", player.level);
}

fn update_time(time: Res<Time>, mut time_query: Query<&mut Text, With<TimeText>>) {
    let Ok(mut time_text) = time_query.get_single_mut() else {
        return;
    };

    let total_seconds = time.elapsed_secs_f64().round() as u32;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    **time_text = format!("Time: {:02}:{:02}", minutes, seconds);
}
