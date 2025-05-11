mod camera_plugin;
mod cube;
mod cube_chain;
mod player;
mod robot_constructor;
mod ui;

use ui::ui_plugin::MyUiPlugin;
//use avian2d::prelude::*;
use bevy::{
    color::palettes::{
        css::BLUE_VIOLET,
        tailwind::{BLUE_950, GREEN_800, RED_700, VIOLET_900, YELLOW_50},
    },
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use camera_plugin::CameraPlugin;
use cube::Cube;
use player::player_plugin::PlayerPlugin;
use rand::{self, random_range};
use robot_constructor::RobotConstructorPlugin;

fn main() {
    App::new()
        .init_resource::<MyTimer>()
        .init_resource::<NumberOfEntity>()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .add_plugins(MyUiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)) // rapier2d
        .add_plugins(RapierDebugRenderPlugin::default()) // rapier2d
        //.add_plugins(FpsCounterPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(RobotConstructorPlugin)
        .add_systems(Startup, spawn_terrain)
        .add_systems(PostUpdate, despawn_when_surpass_lower_bound_cond)
        .add_observer(on_cube_spawn)
        .add_observer(on_cube_despawn)
        .run();
}

#[derive(Resource, Default)]
pub struct NumberOfEntity(pub u32);

pub fn on_cube_spawn(
    event: Trigger<OnAdd, Cube>,
    mut number_of_entity: ResMut<NumberOfEntity>,
    mut commands: Commands,
) {
    number_of_entity.0 += 1;
    let entity = event.target();
    commands
        .entity(entity)
        .observe(
            |out: Trigger<Pointer<Over>>,
             mut materials: ResMut<Assets<ColorMaterial>>,
             mut commands: Commands| {
                let entity = out.target();
                commands.entity(entity).insert(MeshMaterial2d(
                    materials.add(ColorMaterial::from_color(GREEN_800)),
                ));
            },
        )
        .observe(
            |out: Trigger<Pointer<Out>>,
             mut materials: ResMut<Assets<ColorMaterial>>,
             mut commands: Commands| {
                let entity = out.target();
                commands.entity(entity).insert(MeshMaterial2d(
                    materials.add(ColorMaterial::from_color(BLUE_950)),
                ));
            },
        );

    println!("n cube: {}", number_of_entity.0);
}
pub fn on_cube_despawn(
    _event: Trigger<OnRemove, Cube>,
    mut number_of_entity: ResMut<NumberOfEntity>,
) {
    number_of_entity.0 -= 1;
    println!("n cube: {}", number_of_entity.0);
}

#[derive(Resource)]
struct MyTimer(Timer);
impl Default for MyTimer {
    fn default() -> Self {
        MyTimer(Timer::from_seconds(0.001, TimerMode::Once))
    }
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
        LinearRgba::from(BLUE_VIOLET),
    ];

    for i in -100..100 {
        let i = i as f32;
        let color = &colors[random_range(0..5)].with_luminance(1.);
        let x = width * i - 200.0;
        let y = x.abs() * 0.3 - 100.0;
        commands.spawn((
            Terrain,
            RigidBody::Fixed,
            Collider::cuboid(width / 2.0, width / 2.0),
            Transform::from_xyz(x, y, 0.0),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(*color))),
            Mesh2d(meshes.add(Rectangle::new(width, width))),
        ));
    }
}

fn despawn_when_surpass_lower_bound_cond(
    par_commands: ParallelCommands,
    mut commands: Commands,
    q: Query<(Entity, &Transform)>,
) {
    let noe = q.iter().len();
    let height = -7400.0;
    match noe {
        ..=600 => {
            q.iter().for_each(|(entity, transform)| {
                if transform.translation.y < height {
                    commands.entity(entity).despawn();
                }
            });
        }
        _ => {
            q.par_iter().for_each(|(entity, transform)| {
                if transform.translation.y < height {
                    par_commands.command_scope(|mut commands| commands.entity(entity).despawn())
                }
            });
        }
    }
}
fn _despawn_when_surpass_lower_bound_single(
    mut commands: Commands,
    q: Query<(Entity, &Transform)>,
) {
    let height = -400.0;
    q.iter().for_each(|(entity, transform)| {
        if transform.translation.y < height {
            commands.entity(entity).despawn();
        }
    });
}
fn _despawn_when_surpass_lower_bound_par(
    par_commands: ParallelCommands,
    q: Query<(Entity, &Transform)>,
) {
    q.par_iter().for_each(|(entity, transform)| {
        if transform.translation.y < -400.0 {
            par_commands.command_scope(|mut commands| commands.entity(entity).despawn())
        }
    });
}
