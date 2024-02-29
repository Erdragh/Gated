use bevy::prelude::{Query, Res, Time, Transform};

use crate::components::movement::{MovementTarget, Speed};

pub fn movement(mut query: Query<(&mut Transform, &Speed)>, time: Res<Time>) {
    for (mut transform, speed) in &mut query {
        transform.translation.y += speed.y * time.delta_seconds();
        transform.translation.x += speed.x * time.delta_seconds();
    }
}

pub fn move_to_targets(mut query: Query<(&mut Speed, &Transform, &MovementTarget)>) {
    for (mut speed, transform, target) in &mut query {
        let delta_x = target.x - transform.translation.x;
        let delta_y = target.y - transform.translation.y;
        let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
        if distance < 0.1 {
            speed.x = 0.0;
            speed.y = 0.0;
            continue
        }
        speed.x = delta_x / distance * speed.base;
        speed.y = delta_y / distance * speed.base;
    }
}