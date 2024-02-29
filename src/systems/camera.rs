use bevy::math::Vec3;
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
    let party_count = party.iter().count();
    let sum = party.iter().map(|x| x.translation).sum::<Vec3>();
    let avg = sum / (party_count as f32);

    match camera.get_single_mut() {
        Ok(mut transform) => {
            let diff = avg - transform.translation;
            transform.translation += diff * time.delta_seconds() * 4.0;
        }
        Err(_) => {}
    }
}
