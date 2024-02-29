use bevy::prelude::Query;

use crate::components::entity::{Character, Effect, Effects, Health};

pub mod camera;
pub mod world;
pub mod party;
pub mod movement;
pub mod input;
pub mod turn;

pub fn apply_effects(mut query: Query<(&mut Health, &mut Effects, &mut Character)>) {
    for (mut health, effects, character) in query.iter_mut() {
        for Effect {
            action,
            id: _,
            duration: _
        } in &effects.0 {
            action(&mut health);
        }
    }
}