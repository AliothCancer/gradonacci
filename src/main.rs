mod camera_plugin;

use bevy::{color::palettes::{css::BLUE_VIOLET, tailwind::{BLUE_600, BLUE_950, GREEN_800, RED_700, VIOLET_900, YELLOW_50}}, prelude::*, transform};
use camera_plugin::CameraPlugin;
use rand::{self, random_range};
use avian2d::prelude::*;

#[derive(Resource)]
struct MyTimer(Timer);

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 5.;
#[derive(Component)]
struct Player;

impl Default for MyTimer {
    fn default() -> Self {
        MyTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

fn main() {
    App::new()
        .init_resource::<MyTimer>()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, spawn_line)
        .add_systems(Startup, spawn_terrain)
        .add_systems(Update, input_handling)
        .add_systems(Update, move_line)
        .run();
}

fn input_handling(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut line_transform: Query<&mut Transform, With<Line>>,
    mut timer: ResMut<MyTimer>,
    time: Res<Time>,
) {
    if kb_input.pressed(KeyCode::KeyR) {
        line_transform.iter_mut().for_each(|mut trans| {
            let dt = 90f32.to_radians() * time.delta_secs();
            trans.rotate_y(dt);
            trans.rotate_x(dt);
            trans.rotate_z(dt);
        });
        println!("rotate");
    }
    timer.0.tick(time.delta());
}

#[derive(Component)]
struct Terrain;
fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let width = 20.0;
    let colors = [
        LinearRgba::from(BLUE_950),
        LinearRgba::from(GREEN_800),
        LinearRgba::from(YELLOW_50),
        LinearRgba::from(RED_700),
        LinearRgba::from(VIOLET_900),
        LinearRgba::from(BLUE_VIOLET)
    ];
    
    for i in 0..255 {
        let i = i as f32;
        let color = &colors[random_range(0..5)].with_luminance(1.);
        
        commands.spawn((
            Terrain,
            RigidBody::Static,
            Collider::rectangle(width, width),
            Transform::from_xyz(1.2*width*i-200.0, -100.0, 0.0),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(*color))),
            Mesh2d(meshes.add(Rectangle::new(width, width))),
        ));
    }
}

#[derive(Component)]
struct Line;
fn spawn_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::linear_rgb(20., 0.0, 0.0);
    commands.spawn((
        Line,
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
        Mesh2d(meshes.add(Rectangle::new(10.0, 100.0))),
    ));
}

fn move_line(
    mut transform: Query<&mut Transform, With<Line>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut trs = transform.single_mut().unwrap();
    if kb_input.pressed(KeyCode::KeyW) {
        trs.translation.y += PLAYER_SPEED;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        trs.translation.y -= PLAYER_SPEED;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        trs.translation.x -= PLAYER_SPEED;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        trs.translation.x += PLAYER_SPEED;
    }
}
