use avian2d::prelude::*;
use bevy::prelude::*;

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
            TransformInterpolation,
            Collider::rectangle(width, height),
            LinearVelocity::ZERO,
            // initial position
            Transform::from_xyz(0.0, 0.0, 0.0),
        )
    }
}
