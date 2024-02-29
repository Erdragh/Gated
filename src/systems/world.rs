use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;

pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("world.png"),
        ..default()
    });
}