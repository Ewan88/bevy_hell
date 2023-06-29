use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const MAP_SIZE: f32 = 1280.;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_map);
    }
}

fn setup_map(mut commands: Commands) {
    commands.spawn((
        Collider::cuboid(5.0, MAP_SIZE),
        TransformBundle::from(Transform::from_xyz(-MAP_SIZE, 0., 0.)),
    ));
    commands.spawn((
        Collider::cuboid(5.0, MAP_SIZE),
        TransformBundle::from(Transform::from_xyz(MAP_SIZE, 0., 0.)),
    ));
    commands.spawn((
        Collider::cuboid(MAP_SIZE, 5.0),
        TransformBundle::from(Transform::from_xyz(0., MAP_SIZE, 0.)),
    ));
    commands.spawn((
        Collider::cuboid(MAP_SIZE, 5.0),
        TransformBundle::from(Transform::from_xyz(0., -MAP_SIZE, 0.)),
    ));
}
