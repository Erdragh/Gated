use bevy::prelude::{MouseButton, Res, ButtonInput, Query, With, Transform, OrthographicProjection};
use bevy::render::camera::ScalingMode;
use bevy::window::{PrimaryWindow, Window};
use log::info;
use rand::Rng;
use crate::components::entity::{GatedCamera, PartyMember};
use crate::components::movement::MovementTarget;

pub fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
    mut party: Query<&mut MovementTarget, With<PartyMember>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Transform, &OrthographicProjection), With<GatedCamera>>
) {
    let mut rng = rand::thread_rng();
    let window = window.single();
    let (position, projection) = camera.single();
    let scale = match projection.scaling_mode {
        ScalingMode::WindowSize(x) => x,
        ScalingMode::FixedVertical(x) => window.resolution.height() / x,
        _ => todo!()
    };
    if let Some(pos) = window.cursor_position() {
        let x = (-window.resolution.width() / 2.0 + pos.x) / scale + position.translation.x;
        let y = (window.resolution.height() / 2.0 - pos.y) / scale + position.translation.y;
        if buttons.just_pressed(MouseButton::Left) {
            info!("Mouse Click: {}", pos);
            info!("Window Resolution: w={} h={}; scale: {}", window.resolution.width(), window.resolution.height(), scale);
            info!("Mouse press {} {}", x, y);
            for mut target in &mut party {
                let offset_x: f32 = rng.gen();
                let offset_y: f32 = rng.gen();
                target.x = x + (offset_x - 0.5) * 32.0;
                target.y = y + (offset_y - 0.5) * 32.0;
            }
        }
    }
}