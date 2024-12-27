use crate::GameState;

use super::components::Player;
use bevy::{color, prelude::*};

#[derive(Component)]
pub struct LevelUpMenu;

#[derive(Component)]
pub enum MenuButtonAction {
    AttackSpeed,
    MovementSpeed,
    Health,
}

pub fn gain_level(
    mut player_query: Query<&mut Player>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    if player.xp >= player.next_level {
        player.level += 1;
        player.next_level = player.xp_to_next_level();
        game_state.set(GameState::Paused);
    }
}

pub fn spawn_levelup_menu(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor::from(Color::srgba(0., 0., 0., 0.5)),
            LevelUpMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor::from(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Level Up!"),
                        TextFont {
                            font_size: 50.0,
                            ..default()
                        },
                        TextColor(Color::from(color::palettes::basic::RED)),
                    ));

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(Color::from(color::palettes::basic::GREEN)),
                            MenuButtonAction::AttackSpeed,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Attack Speed +10%"),
                                button_text_font.clone(),
                                TextColor(Color::BLACK),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(Color::from(color::palettes::basic::GREEN)),
                            MenuButtonAction::MovementSpeed,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Movement Speed +25%"),
                                button_text_font.clone(),
                                TextColor(Color::BLACK),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(Color::from(color::palettes::basic::GREEN)),
                            MenuButtonAction::Health,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Health +25"),
                                button_text_font.clone(),
                                TextColor(Color::BLACK),
                            ));
                        });
                });
        });
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_query: Query<&mut Player>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::AttackSpeed => {
                    player.attack_speed_mod += 0.1;
                    game_state.set(GameState::Running);
                }
                MenuButtonAction::MovementSpeed => {
                    player.movement_speed_mod += 0.25;
                    game_state.set(GameState::Running);
                }
                MenuButtonAction::Health => {
                    player.max_health += 25.0;
                    game_state.set(GameState::Running);
                }
            }
        }
    }
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<LevelUpMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
