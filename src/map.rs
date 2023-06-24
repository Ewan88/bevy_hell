use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(draw_map);
    }
}

pub fn draw_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(1280.)),
        material: materials.add(ColorMaterial::from(Color::Rgba {
            red: 0.,
            green: 0.10,
            blue: 0.,
            alpha: 1.,
        })),
        ..Default::default()
    });
}
