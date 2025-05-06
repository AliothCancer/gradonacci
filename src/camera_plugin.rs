#![allow(clippy::type_complexity)]

use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::input::mouse::{AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;

use crate::Line;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 4.5;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // 1. HDR is required for bloom
            ..default()
        },
        Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        Bloom::default(),           // 3. Enable bloom for the camera
    ));
}

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Line>)>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    player_pos: Query<&Transform, (With<Line>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let mut cam_transform = camera_query.single_mut().unwrap();
    let player_transform = player_pos.single().unwrap();

    let Vec3 { x, y, .. } = player_transform.translation;
    let direction = Vec3::new(x, y, cam_transform.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    cam_transform
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());

    // CAMERA ZOOM SCROLL
    let vel = 5.8;
    let scroll_unit = mouse_scroll.delta.y;
    if scroll_unit.abs() > 0.0 {
        let delta = scroll_unit * time.delta_secs() * vel;
        let (x, y, z) = (
            cam_transform.scale.x,
            cam_transform.scale.y,
            cam_transform.scale.z,
        );
        cam_transform.scale = cam_transform.scale.lerp(
            Vec3 {
                x: x - delta,
                y: y - delta,
                z,
            },
            0.9,
        );
    }

    //println!("x:{}, y{}", cam_transform.scale.x, cam_transform.scale.y)
}
