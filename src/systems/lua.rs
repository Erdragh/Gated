use std::process::exit;
use bevy::asset::Assets;
use bevy::prelude::{Query, Res};
use log::error;
use mlua::prelude::LuaNil;
use crate::assets::lua::LuaScript;
use crate::components::lua::Scriptable;
use crate::resources::LuaRuntime;

pub fn tick_scriptables(scriptables: Query<&Scriptable>, loaded_scripts: Res<Assets<LuaScript>>, runtime: Res<LuaRuntime>) {
    for scriptable in &scriptables {
        if let Some(tick_script) = &scriptable.tick_script {
            if let Some(loaded_script) = loaded_scripts.get(tick_script) {
                match runtime.0.lock() {
                    Ok(guard) => {
                        match guard.globals().set("entity", 1) {
                            Err(error) => {
                                error!("Failed to prepare Lua runtime for tick: {}", error);
                                exit(1);
                            }
                            _ => {}
                        }
                        match guard.load(&loaded_script.0).exec() {
                            Err(error) => {
                                error!("Failed to tick: {}", error);
                                exit(1);
                            }
                            _ => {}
                        }
                        match guard.globals().set("entity", LuaNil) {
                            Err(error) => {
                                error!("Failed to clean Lua runtime after tick: {}", error);
                                exit(1);
                            }
                            _ => {}
                        }
                    }
                    Err(error) => {
                        error!("Failed to lock Lua runtime for tick: {}", error);
                        exit(1);
                    }
                }
            }
        }
    }
}