use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct PartyListInterface;

#[derive(Component)]
pub struct PartyMemberButton(pub Entity);