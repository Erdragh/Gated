use std::collections::HashMap;

use bevy::prelude::{Commands, Query, With};
use bevy::utils::Uuid;
use log::{info, log, Log, logger};

use crate::components::entity::{Character, Effect, Effects, Health, HealthModifier, PartyMember};
use crate::components::resources::Resources;

pub mod camera;

pub fn spawn_party(mut commands: Commands) {
    for i in 0..4 {
        let map = HashMap::from([
            ("Actions".to_string(), 1u8),
            ("Bonus Actions".to_string(), 1),
            ("Movement".to_string(), 9)
        ]);
        let mut health = Health::new();
        health.modifiers.push(HealthModifier {
            amount: 10,
            id: Uuid::new_v4(),
        });
        commands.spawn((
            Character {
                id: Uuid::new_v4(),
                name: format!("Player {i}"),
                level: 1
            },
            health,
            PartyMember,
            Resources(map),
            Effects(Vec::new())
        ));
    }
}

pub fn heal_party(mut query: Query<&mut Health, With<PartyMember>>) {
    for mut health in query.iter_mut() {
        let max = health.get_max();
        health.set(max);
    }
}

pub fn apply_poison(mut query: Query<&mut Effects>) {
    for mut effects in query.iter_mut() {
        effects.0.push(Effect {
            id: Uuid::new_v4(),
            duration: 0,
            action: |health| {
                health.modify(-1);
            }
        });
    }
}

pub fn apply_effects(mut query: Query<(&mut Health, &mut Effects, &mut Character)>) {
    for (mut health, effects, character) in query.iter_mut() {
        info!("Character: {}", character.name);
        for Effect {
            action,
            id: _,
            duration: _
        } in &effects.0 {
            action(&mut health);
            println!("Health: {}", health.get_current())
        }
    }
}