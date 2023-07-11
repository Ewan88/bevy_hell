use bevy::prelude::*;

#[derive(Resource)]
pub struct Images {
    pub samurai: Handle<Image>,
    pub blob: Handle<Image>,
    pub slash_attack: Handle<Image>,
}

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_images);
    }
}

fn setup_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Images {
        samurai: asset_server.load("samurai.png"),
        blob: asset_server.load("blob.png"),
        slash_attack: asset_server.load("slash_attack.png"),
    });
}
