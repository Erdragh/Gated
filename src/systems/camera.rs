use bevy::prelude::{Camera2dBundle, Commands, Transform};
use bevy::utils::default;
use crate::components::entity::GatedCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        GatedCamera
    ));
}