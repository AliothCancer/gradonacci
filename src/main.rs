mod camera_plugin;
mod player_plugin;
mod cube;

use bevy::{color::palettes::{css::BLUE_VIOLET, tailwind::{BLUE_950, GREEN_800, RED_700, VIOLET_900, YELLOW_50}}, prelude::*};
use camera_plugin::CameraPlugin;
use player_plugin::{PlayerPlugin, Line};
use rand::{self, random_range};
use avian2d::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;

#[derive(Resource)]
struct MyTimer(Timer);



impl Default for MyTimer {
    fn default() -> Self {
        MyTimer(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

fn main() {
    App::new()
        .init_resource::<MyTimer>()
        .add_plugins((DefaultPlugins,PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec2 { x: 0.0, y: -9.81*100.0 }))
        .add_plugins(FpsCounterPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_terrain)
        .run();
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


