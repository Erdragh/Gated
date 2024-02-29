use bevy::prelude::{Camera2dBundle, Commands, Query, Res, Transform, With, Without};
use bevy::render::camera::ScalingMode;
use bevy::time::Time;

use crate::components::entity::{GatedCamera, PartyMember};
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

pub fn follow_cam(mut camera: Query<&mut Transform, (With<GatedCamera>, Without<PartyMember>)>, party: Query<&Transform, With<PartyMember>>, time: Res<Time>) {
    let mut x= 0.0;
    let mut y = 0.0;
    for Transform { translation, rotation: _, scale: _} in &party {
        x += translation.x;
        y += translation.y;
    }
    let party_count = party.iter().count();
    x /= party_count as f32;
    y /= party_count as f32;
    match camera.get_single_mut() {
        Ok(mut transform) => {
            let x_diff = x - transform.translation.x;
            let y_diff = y - transform.translation.y;
            transform.translation.y += y_diff * time.delta_seconds() * 4.0;
            transform.translation.x += x_diff * time.delta_seconds() * 4.0;
        }
        Err(_) => {}
    }
}