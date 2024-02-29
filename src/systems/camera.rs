use bevy::prelude::{Camera2dBundle, Commands, Query, Res, Transform, With};
use bevy::render::camera::ScalingMode;
use bevy::time::Time;

use crate::components::entity::GatedCamera;
use crate::resources::CameraInfo;

pub fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(128.0);
    let info = CameraInfo {
        id: commands.spawn((
            camera,
            GatedCamera
        )).id()
    };
    commands.insert_resource(info);
}

pub fn follow_cam(mut query: Query<&mut Transform, With<GatedCamera>>, time: Res<Time>) {
    match query.get_single_mut() {
        Ok(mut transform) => {
            transform.translation.y += 10.0 * time.delta_seconds()
        }
        Err(_) => {}
    }
}