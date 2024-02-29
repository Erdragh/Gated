use std::time::Duration;

use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};
use bevy::sprite::SpriteBundle;
use bevy::time::Timer;
use bevy::utils::default;

use crate::resources::TurnTimer;

pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TurnTimer(Timer::new(
        Duration::from_secs(6),
        bevy::time::TimerMode::Repeating,
    )));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("world.png"),
        ..default()
    });
}
