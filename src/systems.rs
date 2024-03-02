use bevy::prelude::Query;

use crate::components::entity::{Character, Effect, Effects, Health};

pub mod camera;
pub mod input;
pub mod lua;
pub mod movement;
pub mod party;
pub mod turn;
pub mod ui;
pub mod world;

pub fn apply_effects(mut query: Query<(&mut Health, &mut Effects, &mut Character)>) {
    for (mut health, effects, character) in query.iter_mut() {
        for Effect {
            action,
            id: _,
            duration: _,
        } in &effects.0
        {
            action(&mut health);
        }
    }
}

