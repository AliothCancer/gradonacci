use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{cube::Cube, MyTimer};

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 10_000.;
#[derive(Component)]
struct Player {
    mouse_coor: (f32, f32),
    n_cube: u32,
}

#[derive(Component)]
pub struct Line;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, spawn_cube_skill)
            .add_systems(Update, my_cursor_system);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (width, height) = (10.0, 100.0);
    let color = Color::linear_rgb(20., 0.0, 0.0);
    commands.spawn((
        // unique tags
        Player {
            mouse_coor: (0.0, 0.0),
            n_cube: 0,
        },
        Line,
        // physics
        RigidBody::Dynamic,
        TransformInterpolation,
        Collider::rectangle(width, height),
        LinearVelocity::ZERO,
        // initial position
        Transform::from_xyz(0.0, 0.0, 0.0),
        // appeareance
        Mesh2d(meshes.add(Rectangle::new(width, height))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
    ));
}

fn move_player(
    mut transform: Query<&mut LinearVelocity, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    spawn_timer: Res<MyTimer>
) {
    //println!("timer: {}", spawn_timer.0.elapsed_secs());
    let delta_secs = time.delta_secs();
    //spawn_timer.0.tick(Duration::from_secs_f32(delta_secs));
    let mut linear_vel = transform.single_mut().unwrap();
    if kb_input.pressed(KeyCode::KeyW) {
        linear_vel.y += PLAYER_SPEED * delta_secs;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        linear_vel.y -= PLAYER_SPEED * delta_secs;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        linear_vel.x -= PLAYER_SPEED * delta_secs;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        linear_vel.x += PLAYER_SPEED * delta_secs;
    }

    *linear_vel = LinearVelocity::from(linear_vel.lerp(Vec2::ZERO, 0.2));
}

fn my_cursor_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut player: Query<&mut Player>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single().unwrap();

    if let Some(world_position) = window
        .unwrap()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        //eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        let mut player = player.single_mut().unwrap();
        player.mouse_coor = (world_position.x, world_position.y)
    }
}

fn spawn_cube_skill(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut player: Query<&mut Player>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut spawn_timer: ResMut<MyTimer>,
    time: Res<Time>,
) {
    
    let delta_time = time.delta_secs();
    
    spawn_timer.0.tick(Duration::from_secs_f32(delta_time));
    
    if mouse.pressed(MouseButton::Left) && spawn_timer.0.finished() {
        spawn_timer.0.reset();
        let (x_spawn, y_spawn) = player.single().unwrap().mouse_coor;
        commands.spawn(Cube::bundle(30.0, 30.0)).insert((
            Transform::from_xyz(x_spawn, y_spawn, 0.0),
            // appeareance
            Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(LinearRgba::rgb(0.0, 0.0, 1.0))),
            ),
        ));
        let mut pl = player.single_mut().unwrap();
        pl.n_cube += 1;
        println!("n cube: {}",pl.n_cube );
    }
}
