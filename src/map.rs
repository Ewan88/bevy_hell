use bevy::prelude::*;

#[allow(dead_code)]
const MAP_SIZE: f32 = 1280.;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

#[allow(unused_variables, unused_mut)]
fn setup_map(mut commands: Commands) {}
