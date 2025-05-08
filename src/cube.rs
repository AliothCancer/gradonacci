//use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Cube {
    pub width: f32,
    pub height: f32,
}

impl Cube {
    pub fn bundle(width: f32, height: f32) -> impl Bundle {
        (
            Self { width, height },
            // physics
            RigidBody::Dynamic,
            //TransformInterpolation,
            //LinearVelocity::ZERO,
            // initial position
            Transform::from_xyz(0.0, 0.0, 0.0),
        )
    }
}
