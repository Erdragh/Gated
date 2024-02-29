#[macro_use]
extern crate luajit;

mod components;
mod systems;
mod lua;
mod resources;

use bevy::prelude::*;
use crate::lua::test_lua;
use crate::systems::*;
use crate::systems::camera::{follow_cam, setup_camera};
use crate::systems::world::setup_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (
            test_lua,
            spawn_party,
            setup_world,
            heal_party.after(spawn_party),
            apply_poison.after(spawn_party).after(heal_party),
            setup_camera
        ))
        .add_systems(Update, (apply_effects, gravity, movement, follow_cam))
        .run();
}