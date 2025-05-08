use std::time::Duration;

//use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{cube::Cube, robot_constructor::Shape, MyTimer, Terrain};

use super::mouse_interaction_plugin::{add_child, MouseInteractionPlugin, PlayerMouseCoor};

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 10_000.;

#[derive(Component)]
pub struct Line;

#[derive(Event)]
struct ChangeShape;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(MouseInteractionPlugin)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, update_mouse_player_coor)
            .add_systems(FixedUpdate, (change_shape, move_player).chain())
            .add_systems(Update, (spawn_cube_skill, despawn_cube_skill))
            .add_systems(Update, add_child);
    }
}

#[derive(Component)]
struct Player;

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
            Player,
            Line,
            // physics
            RigidBody::Dynamic,
            //TransformInterpolation,
            //LinearVelocity::ZERO,
            Velocity::zero(),
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
    mut transform: Query<&mut Velocity, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    _spawn_timer: Res<MyTimer>,
) {
    //println!("timer: {}", spawn_timer.0.elapsed_secs());
    let delta_secs = time.delta_secs();
    //spawn_timer.0.tick(Duration::from_secs_f32(delta_secs));
    let mut transf = transform.single_mut().unwrap();

    if kb_input.pressed(KeyCode::KeyW) {
        transf.linvel.y += PLAYER_SPEED * delta_secs;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        transf.linvel.y -= PLAYER_SPEED * delta_secs;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        transf.linvel.x -= PLAYER_SPEED * delta_secs;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        transf.linvel.x += PLAYER_SPEED * delta_secs;
    }

    transf.linvel = transf.linvel.lerp(Vec2::ZERO, 0.2);
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
        }
    }
}
fn spawn_cube_skill(
    mut commands: Commands,
    mouse_coor: Res<PlayerMouseCoor>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    kbd_input: Res<ButtonInput<KeyCode>>,
    mut spawn_timer: ResMut<MyTimer>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    spawn_timer.0.tick(Duration::from_secs_f32(delta_time));

    if mouse_input.pressed(MouseButton::Left)
        && kbd_input.pressed(KeyCode::KeyP)
        && spawn_timer.0.finished()
    {
        spawn_timer.0.reset();
        let (x_spawn, y_spawn) = (mouse_coor.x, mouse_coor.y);
        let _id = commands
            .spawn(Cube::bundle(30.0, 30.0))
            .insert((
                Transform::from_xyz(x_spawn, y_spawn, 0.0),
                Shape::Circle { radius: 20.0 },
                RigidBody::Dynamic,
                //TransformInterpolation,
                Velocity::zero(),
            ))
            //.observe(on_cube_spawn) attacca un local observer all'entity
            .id();
        //commands.trigger_targets(SpawnedAcube, id);

        //let mut pl = player.single_mut().unwrap();
        //pl.n_cube += 1;
        //println!("n cube: {}", pl.n_cube);
    }
}
