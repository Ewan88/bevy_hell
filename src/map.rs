use bevy::prelude::*;

const MAP_SIZE: f32 = 1280.;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_map);
    }
}

fn setup_map(mut commands: Commands) {}
