use std::process::exit;
use bevy::asset::Assets;
use bevy::prelude::{Query, Res};
use log::error;
use crate::assets::lua::LuaScript;
use crate::components::lua::Scriptable;
use crate::resources::LuaRuntime;

pub fn tick_scriptables(scriptables: Query<&Scriptable>, loaded_scripts: Res<Assets<LuaScript>>, runtime: Res<LuaRuntime>) {
    for scriptable in &scriptables {
        if let Some(tick_script) = &scriptable.tick_script {
            if let Some(loaded_script) = loaded_scripts.get(tick_script) {
                match runtime.0.lock().unwrap().load(&loaded_script.0).exec() {
                    Ok(_) => {}
                    Err(error) => {
                        error!("Failed to run tick script: {}", error);
                        exit(1);
                    }
                }
            }
        }
    }
}