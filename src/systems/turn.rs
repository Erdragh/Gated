use bevy::{
    ecs::system::{Query, Res, ResMut},
    time::Time,
};
use log::info;

use crate::{components::movement::TurnBasedMovement, resources::TurnTimer};

pub fn turn(
    time: Res<Time>,
    mut turn_timer: ResMut<TurnTimer>,
    mut movement: Query<&mut TurnBasedMovement>,
) {
    turn_timer.0.tick(time.delta());
    if !turn_timer.0.just_finished() {
        return;
    }
    info!("Turn!");
    for mut instance in &mut movement {
        instance.accu = 0.0;
    }
}
