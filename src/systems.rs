use std::collections::HashMap;

use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Commands, Query, Res, SpatialBundle, Transform, With};
use bevy::sprite::SpriteBundle;
use bevy::time::Time;
use bevy::utils::{default, Uuid};
use log::Log;

use crate::components::entity::{Character, Effect, Effects, Health, HealthModifier, Mass, PartyMember, Speed};
use crate::components::resources::Resources;

pub mod camera;
pub mod world;

pub fn spawn_party(mut commands: Commands, asset_server: Res<AssetServer>) {
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

        let sprite = commands.spawn(SpriteBundle {
            texture: asset_server.load("test.png"),
            ..default()
        }).id();

        let parent = commands.spawn((
            Character {
                id: Uuid::new_v4(),
                name: format!("Player {i}"),
                level: 1,
            },
            SpatialBundle::from_transform(Transform::from_xyz(i as f32 * 32.0, 0.0, 0.0)),
            Mass(32.0),
            Speed {
                x: 0.0,
                y: 0.0
            },
            health,
            PartyMember,
            Resources(map),
            Effects(Vec::new())
        )).id();
        commands.entity(parent).push_children(&[sprite]);
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
            },
        });
    }
}

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

pub fn gravity(mut query: Query<&mut Speed, (With<Mass>, With<Transform>)>, time: Res<Time>) {
    for mut speed in &mut query {
        //speed.y += -9.81 * time.delta_seconds();
    }
}

pub fn movement(mut query: Query<(&mut Transform, &Speed)>, time: Res<Time>) {
    for (mut transform, speed) in &mut query {
        transform.translation.y += speed.y * time.delta_seconds();
        transform.translation.x += speed.x * time.delta_seconds();
    }
}