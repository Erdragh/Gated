use bevy::prelude::{Camera2dBundle, Commands};
use bevy::render::camera::ScalingMode;

use crate::components::entity::GatedCamera;

pub fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(128.0);
    commands.spawn((
        camera,
        GatedCamera
    ));
}