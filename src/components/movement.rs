use std::fmt::{Display, Formatter};
use bevy::prelude::Component;

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
    pub base: f32
}

impl Display for Speed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Speed: (x: {}, y: {}, base: {})", self.x, self.y, self.base)
    }
}

#[derive(Component)]
pub struct MovementTarget {
    pub x: f32,
    pub y: f32,
}