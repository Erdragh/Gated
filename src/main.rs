use std::process::exit;

use bevy::prelude::*;

use crate::lua::test_lua;
use crate::systems::*;
use crate::systems::camera::{follow_cam, setup_camera};
use crate::systems::input::mouse::mouse_button_input;
use crate::systems::movement::{move_to_targets, movement};
use crate::systems::party::{apply_poison, heal_party, spawn_party};
use crate::systems::turn::turn;
use crate::systems::world::setup_world;

mod components;
mod systems;
mod lua;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (
            || {
                match test_lua() {
                    Ok(_) => {}
                    Err(x) => {
                        error!("{}", x);
                        exit(1);
                    }
                }
            },
            spawn_party,
            setup_world,
            heal_party.after(spawn_party),
            apply_poison.after(spawn_party).after(heal_party),
            setup_camera
        ))
        .add_systems(Update, (
            apply_effects,
            movement,
            follow_cam,
            move_to_targets,
            mouse_button_input,
            turn
        ))
        .run();
}
