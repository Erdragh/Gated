use std::collections::HashMap;

use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Commands, Query, Res, SpatialBundle, Transform, With};
use bevy::sprite::SpriteBundle;
use bevy::time::Time;
use bevy::utils::{default, Uuid};
use log::Log;

use crate::components::entity::{Character, Effect, Effects, Health, HealthModifier, PartyMember};
use crate::components::movement::{Mass, Speed};
use crate::components::resources::Resources;

pub mod camera;
pub mod world;
pub mod party;
pub mod movement;

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