#![allow(clippy::type_complexity)]

use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;

/// Camera lerp factor.
const CAM_LERP_FACTOR: f32 = 3.7;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut orto_proj = OrthographicProjection::default_2d();
    *orto_proj.get_field_mut::<f32>("scale").unwrap() = 1.;
    commands.spawn((
        Camera2d,
        orto_proj,
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
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut _transform, mut camera_projection) = camera_query.single_mut();


    let vel = 1.;
    if kb_input.pressed(KeyCode::NumpadAdd) {
        camera_projection.scale += 1. * time.delta_secs() * vel;
    }
    if kb_input.pressed(KeyCode::NumpadSubtract) {
        camera_projection.scale -= 1. * time.delta_secs() * vel;
    }

}
