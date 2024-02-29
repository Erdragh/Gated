use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct CameraInfo {
    pub id: Entity
}