use bevy::{prelude::*, sprite::Anchor};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, ui_system);
    }
}

fn ui_system(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section("Hello", TextStyle::default()),
        text_anchor: Anchor::TopLeft,
        ..Default::default()
    });
}
