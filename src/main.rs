mod camera_plugin;
use std::f32::consts::PI;

use bevy::prelude::*;
use camera_plugin::CameraPlugin;


#[derive(Resource)]
struct MyTimer(Timer);

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
        .add_systems(Update, input_handling)
        .run();
}

fn input_handling(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut line_transform: Query<&mut Transform, With<Line>>,
    mut timer: ResMut<MyTimer>,
    time: Res<Time>
) { 

    if kb_input.pressed(KeyCode::KeyR){
        line_transform.iter_mut().for_each(|mut trans|{
            let dt = 90f32.to_radians() * time.delta_secs() ;
            trans.rotate_y(dt);
            trans.rotate_x(dt);
            trans.rotate_z(dt);
        });
        println!("rotate");
    }
    timer.0.tick(time.delta());
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
