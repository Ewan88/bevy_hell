use bevy::prelude::*;

#[derive(Resource)]
pub struct Images {
    pub samurai: Handle<Image>,
    pub blob: Handle<Image>,
    pub slash_attack: Handle<Image>,
    pub health_potion: Handle<Image>,
}

#[derive(Resource)]
pub struct Audio {
    pub health_down: Handle<AudioSource>,
    pub slash_attack: Handle<AudioSource>,
    pub background_bleeps: Handle<AudioSource>,
}

#[derive(Component)]
pub struct PlayerHitSound {
    pub timer: Timer,
}

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_images);
        app.add_systems(PreStartup, setup_audio);
    }
}

fn setup_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Images {
        samurai: asset_server.load("samurai.png"),
        blob: asset_server.load("blob.png"),
        slash_attack: asset_server.load("slash_attack.png"),
        health_potion: asset_server.load("health_potion.png"),
    });
}

fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Audio {
        health_down: asset_server.load("health_down.ogg"),
        slash_attack: asset_server.load("slash_attack.ogg"),
        background_bleeps: asset_server.load("background_bleeps.ogg"),
    });
}
