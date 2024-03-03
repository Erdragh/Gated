use std::collections::HashMap;

use bevy::asset::AssetServer;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::Added;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::*;
use bevy::utils::Uuid;

use crate::components::entity::{
    Character, Effect, Effects, Health, HealthModifier, PartyList, PartyMember,
};
use crate::components::lua::Scriptable;
use crate::components::movement::{Mass, MovementTarget, Speed, TurnBasedMovement};
use crate::components::resources::Resources;

pub fn spawn_party(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..4 {
        let map = HashMap::from([
            ("Actions".to_string(), 1u8),
            ("Bonus Actions".to_string(), 1),
            ("Movement".to_string(), 9),
        ]);
        let mut health = Health::new();
        health.modifiers.push(HealthModifier {
            amount: 10,
            id: Uuid::new_v4(),
        });

        let sprite = commands
            .spawn(SpriteBundle {
                texture: asset_server.load("test.png"),
                ..default()
            })
            .id();

        let initial_position = Transform::from_xyz(i as f32 * 32.0, 0.0, 0.0);

        let parent = commands
            .spawn((
                Character {
                    id: Uuid::new_v4(),
                    name: format!("Player {i}"),
                    level: 1,
                },
                SpatialBundle::from_transform(initial_position),
                MovementTarget {
                    x: initial_position.translation.x,
                    y: initial_position.translation.y,
                    active: false,
                },
                TurnBasedMovement {
                    base: 30.0 * 6.0,
                    accu: 0.0,
                },
                Mass(32.0),
                Speed {
                    x: 0.0,
                    y: 0.0,
                    base: 30.0,
                },
                Scriptable {
                    tick_script: Some(asset_server.load("scripts/player_tick.lua")),
                    destroy_script: None,
                    init_script: None,
                    initialized: false,
                },
                health,
                PartyMember,
                Resources(map),
                Effects(Vec::new()),
            ))
            .id();
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

pub fn update_party_lists(
    mut party_lists: Query<&mut PartyList>,
    new_party_members: Query<Entity, Added<PartyMember>>,
    mut removed_party_members: RemovedComponents<PartyMember>,
) {
    for mut list in &mut party_lists {
        for member in &new_party_members {
            let mut new_list = Vec::new();
            new_list.push(member);
            new_list.append(&mut list.list);
            list.list = new_list;
        }
        for member in removed_party_members.read() {
            let new_list: Vec<Entity> = list
                .list
                .iter()
                .filter(|entity| **entity == member)
                .map(|x| *x)
                .collect();
            list.list = new_list;
        }
    }
}
