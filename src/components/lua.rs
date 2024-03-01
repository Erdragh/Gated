use bevy::asset::Handle;
use bevy::prelude::Component;

use crate::assets::lua::LuaScript;

#[derive(Component)]
pub struct Scriptable {
    pub tick_script: Option<Handle<LuaScript>>,
    pub init_script: Option<Handle<LuaScript>>,
    pub initialized: bool,
    pub destroy_script: Option<Handle<LuaScript>>,
}