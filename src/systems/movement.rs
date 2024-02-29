use bevy::prelude::{Query, Res, Time, Transform};

use crate::components::movement::{MovementTarget, Speed, TurnBasedMovement};

pub fn movement(
    mut query: Query<(&mut Transform, &Speed, Option<&mut TurnBasedMovement>)>,
    time: Res<Time>,
) {
    for (mut transform, speed, turn) in query.iter_mut() {
        let mut diff_x = speed.x * time.delta_seconds();
        let mut diff_y = speed.y * time.delta_seconds();
        if let Some(mut turn) = turn {
            if turn.accu >= turn.base {
                return;
            }
            let dist = (diff_x.powi(2) + diff_y.powi(2)).sqrt();
            if dist > turn.base - turn.accu {
                diff_x = diff_x / dist * (turn.base - turn.accu);
                diff_y = diff_y / dist * (turn.base - turn.accu);
                turn.accu = turn.base;
            } else {
                turn.accu += dist;
            }
        }
        transform.translation.y += diff_y;
        transform.translation.x += diff_x;
    }
}

pub fn move_to_targets(
    mut query: Query<(
        &mut Speed,
        &Transform,
        &mut MovementTarget,
    )>,
) {
    for (mut speed, transform, mut target) in query.iter_mut() {
        if !target.active {
            continue;
        }
        let delta_x = target.x - transform.translation.x;
        let delta_y = target.y - transform.translation.y;
        let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
        if distance < 1.0 {
            speed.x = 0.0;
            speed.y = 0.0;
            target.active = false;
            continue;
        }
        speed.x = delta_x / distance * speed.base;
        speed.y = delta_y / distance * speed.base;
    }
}
