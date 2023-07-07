use bevy::prelude::*;

#[derive(Resource)]
pub struct Images {
    pub samurai: Handle<Image>,
    pub blob: Handle<Image>,
}

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_images.in_base_set(StartupSet::PreStartup));
    }
}

fn setup_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Images {
        samurai: asset_server.load("samurai.png"),
        blob: asset_server.load("blob.png"),
    });
}
