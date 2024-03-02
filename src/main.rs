use bevy::prelude::*;
use systems::party::update_party_lists;
use systems::ui::party::{party_ui, setup_party_ui, update_party_ui_members};

use crate::assets::lua::{LuaScript, LuaScriptLoader};
use crate::lua::setup_lua_runtime;
use crate::systems::camera::{follow_cam, setup_camera};
use crate::systems::input::mouse::mouse_button_input;
use crate::systems::lua::tick_scriptables;
use crate::systems::movement::{move_to_targets, movement};
use crate::systems::party::{apply_poison, heal_party, spawn_party};
use crate::systems::turn::turn;
use crate::systems::world::setup_world;
use crate::systems::*;

mod assets;
mod components;
mod lua;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_asset::<LuaScript>()
        .init_asset_loader::<LuaScriptLoader>()
        .add_systems(
            Startup,
            (
                setup_lua_runtime,
                spawn_party,
                setup_world,
                heal_party.after(spawn_party),
                apply_poison.after(spawn_party).after(heal_party),
                setup_camera,
                setup_party_ui,
            ),
        )
        .add_systems(
            Update,
            (
                apply_effects,
                movement,
                follow_cam,
                move_to_targets,
                mouse_button_input,
                turn,
                tick_scriptables,
                party_ui,
                update_party_ui_members,
                update_party_lists,
            ),
        )
        .run();
}
