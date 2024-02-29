mod components;
mod systems;

use bevy::prelude::*;
use crate::systems::*;
use crate::systems::camera::setup_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (
            spawn_party,
            heal_party.after(spawn_party),
            apply_poison.after(spawn_party).after(heal_party),
            setup_camera
        ))
        .add_systems(Update, (apply_effects, gravity, movement))
        .run();
}
