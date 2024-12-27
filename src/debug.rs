use bevy::{color, prelude::*};
use sysinfo::System;

use crate::player::components::Player;

#[derive(Component)]
pub struct DebugText;

#[derive(Resource)]
pub struct SystemInfo {
    pub system: System,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_system_info)
            .add_systems(PostStartup, build_debug_text)
            .add_systems(Update, update_debug_text);
    }
}

fn init_system_info(mut commands: Commands) {
    commands.insert_resource(SystemInfo {
        system: System::new_all(),
    });
}

fn build_debug_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(0.),
            top: Val::Percent(7.5),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 10.,
                    ..default()
                },
                TextColor(Color::from(color::palettes::basic::YELLOW)),
                DebugText,
            ));
        });
}

fn update_debug_text(
    mut text_query: Query<&mut Text, With<DebugText>>,
    player_query: Query<&Transform, With<Player>>,
    mut system_info: ResMut<SystemInfo>,
) {
    let Ok(transform) = player_query.get_single() else {
        return;
    };

    let Ok(mut text) = text_query.get_single_mut() else {
        return;
    };

    system_info.system.refresh_all();

    let memory_used = system_info.system.used_memory() / 1024 / 1024 / 1024;
    let memory_total = system_info.system.total_memory() / 1024 / 1024 / 1024;
    let cpu_usage = system_info.system.global_cpu_usage().floor();

    **text = format!(
        "
        pos: {:.2} {:.2}\n
        {}/{}\n
        {}%
        ",
        transform.translation.x,
        transform.translation.y,
        memory_used,
        memory_total,
        cpu_usage
    );
}
