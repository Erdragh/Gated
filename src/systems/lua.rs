use std::any::Any;
use std::error::Error;
use std::fs::File;
use std::process::exit;
use std::string::FromUtf8Error;
use std::sync::{LockResult, MutexGuard, PoisonError};

use bevy::{
    asset::{AssetLoader, AsyncReadExt, io::Reader, LoadContext, ron},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use bevy::asset::{AssetPath, Assets};
use bevy::prelude::{Query, Res};
use bevy::utils::thiserror;
use log::error;
use mlua::Lua;
use mlua::prelude::{LuaError, LuaNil};
use serde::Deserialize;
use thiserror::Error;

use crate::assets::lua::LuaScript;
use crate::components::lua::Scriptable;
use crate::resources::LuaRuntime;

#[non_exhaustive]
#[derive(Debug, Error)]
enum LuaRuntimeError {
    #[error("Failed to lock Lua runtime: {0}")]
    Lock(String),
    #[error("Runtime failure during script execution: {0}")]
    Run(#[from] LuaError)
}

fn run_entity_script(script: &LuaScript, runtime: &LuaRuntime) -> Result<(), LuaRuntimeError> {
    let guard = match runtime.0.lock() {
        Ok(locked) => locked,
        Err(error) => {
            return Err(LuaRuntimeError::Lock(error.to_string()))
        }
    };
    guard.globals().set("entity", 1)?;
    guard.load(script).exec()?;
    guard.globals().set("entity", LuaNil)?;
    Ok(())
}

pub fn tick_scriptables(scriptables: Query<&Scriptable>, loaded_scripts: Res<Assets<LuaScript>>, runtime: Res<LuaRuntime>) {
    for scriptable in &scriptables {
        if let Some(tick_script) = &scriptable.tick_script {
            if let Some(loaded_script) = loaded_scripts.get(tick_script) {
                match run_entity_script(loaded_script, &runtime) {
                    Err(error) => {
                        error!("Failed to tick entity: {}", error);
                        exit(1);
                    }
                    _ => {}
                }
            }
        }
    }
}