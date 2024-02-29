use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Component)]
pub struct Resources(pub HashMap<String, u8>);