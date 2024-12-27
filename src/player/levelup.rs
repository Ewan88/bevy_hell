use crate::GameState;

use super::components::Player;
use bevy::{color, prelude::*};

#[derive(Component)]
pub struct LevelUpMenu;

#[derive(Component)]
pub struct Button;

#[derive(Component)]
enum MenuButtonAction {
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

    if player.xp > player.next_level {
        player.level += 1;
        player.next_level = player.xp_to_next_level();
        game_state.set(GameState::Paused);
    }
}

pub fn spawn_levelup_menu(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(200.),
        height: Val::Px(50.),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
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
                                Text::new("Attack Speed"),
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
                                Text::new("Movement Speed"),
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
                                Text::new("Health"),
                                button_text_font.clone(),
                                TextColor(Color::BLACK),
                            ));
                        });
                });
        });
}
