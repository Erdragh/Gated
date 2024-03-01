use std::sync::{Arc, Mutex};
use bevy::{prelude::{Entity, Resource}, time::Timer};
use mlua::Lua;

#[derive(Resource)]
pub struct CameraInfo {
    pub id: Entity,
}

#[derive(Resource)]
pub struct TurnTimer(pub Timer);

#[derive(Resource)]
pub struct LuaRuntime(pub Arc<Mutex<Lua>>);