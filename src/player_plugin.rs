use std::time::Duration;

use avian2d::prelude::*;
use bevy::{prelude::*, state::commands};

use crate::{cube::Cube, MyTimer, Terrain};

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 10_000.;
#[derive(Component)]
struct Player {
    n_cube: u32,
}

#[derive(Resource)]
pub struct PlayerMouseCoor {
    x: f32,
    y: f32,
}
impl PlayerMouseCoor {
    fn update(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}
impl Into<Vec2> for &PlayerMouseCoor {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}
impl Default for PlayerMouseCoor {
    fn default() -> Self {
        PlayerMouseCoor { x: 0.0, y: 0.0 }
    }
}

#[derive(Component)]
pub struct Line;

#[derive(Event)]
struct SpawnedAcube;

#[derive(Event)]
struct ChangeShape;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<PlayerMouseCoor>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, update_mouse_player_coor)
            .add_observer(on_cube_spawn)
            .add_observer(on_insert_shape)
            .add_systems(Update, (move_player, change_shape).chain())
            .add_systems(Update, (spawn_cube_skill, despawn_cube_skill));
    }
}

#[derive(Component, Clone, Copy)]
enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

fn on_insert_shape(
    trigger: Trigger<OnInsert, Shape>,
    shape: Query<&Shape>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = trigger.target();
    let Ok(shape) = shape.get(entity) else {
        return;
    };

    let mesh_bundle = match *shape {
        Shape::Circle { radius } => (
            Mesh2d(meshes.add(Circle::new(radius))),
            Collider::circle(radius),
        ),
        Shape::Rectangle { width, height } => (
            Mesh2d(meshes.add(Rectangle::new(width, height))),
            Collider::rectangle(width, height),
        ),
    };

    let color = Color::linear_rgb(1., 0.0, 0.0);
    commands.entity(entity).insert((
        mesh_bundle,
        MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
    ));
}
fn change_shape(
    mut commands: Commands,
    kb_input: Res<ButtonInput<KeyCode>>,
    entity_query: Query<(Entity, &Shape), (With<Player>, With<Shape>)>,
) {
    if kb_input.just_released(KeyCode::KeyG) {
        if let Ok((entity, old_shape)) = entity_query.single() {
            let new_shape = match old_shape {
                Shape::Circle { radius } => Shape::Rectangle {
                    height: *radius,
                    width: *radius,
                },
                Shape::Rectangle { width, height: _ } => Shape::Circle { radius: *width },
            };
            commands.entity(entity).insert(new_shape);
        }
    }
}

fn spawn_player(mut commands: Commands) {
    let id = commands
        .spawn((
            // unique tags
            Player { n_cube: 0 },
            Line,
            // physics
            RigidBody::Dynamic,
            TransformInterpolation,
            LinearVelocity::ZERO,
            Shape::Circle { radius: 34.0 },
            //ColliderDensity(1.0),
            // initial position
            Transform::from_xyz(0.0, 0.0, 0.0),
            // appeareance
        ))
        .id();
    commands.entity(id).trigger(ChangeShape);
}

fn move_player(
    mut transform: Query<&mut LinearVelocity, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    _spawn_timer: Res<MyTimer>,
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

fn update_mouse_player_coor(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut mouse_coor: ResMut<PlayerMouseCoor>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single().unwrap();

    if let Some(world_position) = window
        .unwrap()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        //eprintln!("World coords: {}/{}", world_position.x, world_position.y);

        mouse_coor.update(world_position.x, world_position.y)
    }
}
fn despawn_cube_skill(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Cube), Without<Terrain>>,
    mouse_coor: Res<PlayerMouseCoor>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_input.pressed(MouseButton::Right) {
        //println!("right");
        let mouse_pos: Vec2 = mouse_coor.into_inner().into();

        if let Some((entity, _, _)) = query.iter().find(|(_, trans, cube)| {
            let trans = trans.translation;
            let cube_pos = Vec2 {
                x: trans.x,
                y: trans.y,
            };
            let distance = mouse_pos.distance(cube_pos).abs();
            //println!("{distance}");
            distance < cube.height / 2.0
        }) {
            commands.entity(entity).despawn();
        } else {
            //println!("return");
            return;
        }
    }
}
fn spawn_cube_skill(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mouse_coor: Res<PlayerMouseCoor>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut spawn_timer: ResMut<MyTimer>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    spawn_timer.0.tick(Duration::from_secs_f32(delta_time));

    if mouse_input.pressed(MouseButton::Left) && spawn_timer.0.finished() {
        spawn_timer.0.reset();
        let (x_spawn, y_spawn) = (mouse_coor.x, mouse_coor.y);
        let id = commands
            .spawn(Cube::bundle(30.0, 30.0))
            .insert((
                Transform::from_xyz(x_spawn, y_spawn, 0.0),
                // appeareance
                Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
                MeshMaterial2d(
                    materials.add(ColorMaterial::from_color(LinearRgba::rgb(0.0, 0.0, 1.0))),
                ),
            ))
            //.observe(on_cube_spawn) attacca un local observer all'entity
            .id();
        //commands.trigger_targets(SpawnedAcube, id);

        //let mut pl = player.single_mut().unwrap();
        //pl.n_cube += 1;
        //println!("n cube: {}", pl.n_cube);
    }
}

fn on_cube_spawn(_event: Trigger<OnAdd, Cube>, mut player: Query<&mut Player>) {
    let mut player = player.single_mut().unwrap();
    player.n_cube += 1;
    println!("n cube: {}", player.n_cube);
}
