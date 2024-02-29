use std::path::Path;
use std::process::exit;

use bevy::asset::AssetServer;
use bevy::prelude::Res;
use log::{error, info};
use luajit::{c_int, State};

fn callable_from_lua(state: &mut State) -> c_int {
    info!("Rust Method callable from Lua");
    state.push(42);
    1
}

pub fn test_lua() {
    let mut state = State::new();
    state.open_libs();
    state.push(lua_fn!(callable_from_lua));
    state.set_global("callable_from_lua");

    match state.do_file(Path::new("assets/scripts/lua_test.lua")) {
        Ok(_) => {
            info!("Lua-Test successful");
        }
        Err((status, err)) => {
            error!("Lua-Test failed: {:?} ({})", status, err);
            exit(1);
        }
    }
}