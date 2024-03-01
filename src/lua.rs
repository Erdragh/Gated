use std::sync::{Arc, Mutex};
use bevy::prelude::Commands;

use mlua::Lua;
use crate::resources::LuaRuntime;

pub fn setup_lua_runtime(mut commands: Commands) {
    commands.insert_resource(LuaRuntime(Arc::new(Mutex::new(Lua::new()))))
}