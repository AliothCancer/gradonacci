#![allow(clippy::type_complexity)]

use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
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
const CAMERA_DECAY_RATE: f32 = 2.;



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
    kb_input: Res<ButtonInput<KeyCode>>,
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

    let vel = 0.2;
    if kb_input.pressed(KeyCode::NumpadAdd) {
        cam_transform.scale.x += 1. * time.delta_secs() * vel;
        cam_transform.scale.y += 1. * time.delta_secs() * vel;
    }
    if kb_input.pressed(KeyCode::NumpadSubtract) {
        cam_transform.scale.x -= 1. * time.delta_secs() * vel;
        cam_transform.scale.y -= 1. * time.delta_secs() * vel;
    }
    //println!("x:{}, y{}", cam_transform.scale.x, cam_transform.scale.y)
}
