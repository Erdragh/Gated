use bevy::{prelude::{Entity, Resource}, time::Timer};

#[derive(Resource)]
pub struct CameraInfo {
    pub id: Entity,
}

#[derive(Resource)]
pub struct TurnTimer(pub Timer);