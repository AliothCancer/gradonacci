use std::time::Duration;

//use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random_range;

use crate::{
    cube::Cube,
    robot_constructor::{spawn_robot, EntityColor, Shape},
    MyTimer, Terrain,
};

use super::{mouse_interaction_plugin::MouseInteractionPlugin, resources::PlayerMouseCoor};

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 10_000.;

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
            .add_observer(connect_entities);
    }
}

#[derive(Component)]
pub struct Player(pub ClickMode);

/// Button state for the system SpawnCube -> on_cube_spawn and join_cube observer
#[derive(Clone, Copy, PartialEq)]
pub enum ClickMode {
    SpawnCube,
    JoinCube,
}

/// A component for storing 2 Entity and allow to connect them with a rapier joint
#[derive(Default, Component)]
pub struct PairEntitySelection(pub (Option<Entity>, Option<Entity>));

/// Make the player of another form
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
    let pl_id = spawn_robot(&mut commands);
    // la prima volta che viene aggiunta un Shape viene
    // contata come un evento ChangeShape
    commands
        .entity(pl_id)
        .insert(Player(ClickMode::SpawnCube))
        .insert(PairEntitySelection::default())
        .trigger(ChangeShape);
}

fn move_player(
    mut velocity: Query<&mut Velocity, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    _spawn_timer: Res<MyTimer>,
) {
    //println!("timer: {}", spawn_timer.0.elapsed_secs());
    let delta_secs = time.delta_secs();
    //spawn_timer.0.tick(Duration::from_secs_f32(delta_secs));
    let mut vel = velocity.single_mut().unwrap();

    if kb_input.pressed(KeyCode::KeyW) {
        vel.linvel.y += PLAYER_SPEED * delta_secs;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        vel.linvel.y -= PLAYER_SPEED * delta_secs;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        vel.linvel.x -= PLAYER_SPEED * delta_secs;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        vel.linvel.x += PLAYER_SPEED * delta_secs;
    }

    vel.linvel = vel.linvel.lerp(Vec2::ZERO, 0.2);
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
    _kbd_input: Res<ButtonInput<KeyCode>>,
    mut spawn_timer: ResMut<MyTimer>,
    player: Single<&Player>,
    time: Res<Time>,
) {
    let click_mode = player.0;
    if click_mode == ClickMode::SpawnCube {
        let delta_time = time.delta_secs();

        spawn_timer.0.tick(Duration::from_secs_f32(delta_time));

        if mouse_input.pressed(MouseButton::Left)
            //&& kbd_input.pressed(KeyCode::KeyP)
            && spawn_timer.0.finished()
        {
            spawn_timer.0.reset();
            let (x_spawn, y_spawn) = (mouse_coor.x, mouse_coor.y);
            let rng = || random_range(0.0..1.0);
            commands
                .spawn(Cube::bundle(30.0, 30.0))
                .insert((
                    EntityColor(Color::linear_rgb(rng(), rng(), rng())),
                    Transform::from_xyz(x_spawn, y_spawn, 0.0),
                    Shape::Circle { radius: 20.0 },
                    RigidBody::Dynamic,
                    Velocity::zero(),
                ))
                .observe(join_cube);
            //.observe(on_cube_spawn) attacca un local observer all'entity
            //commands.trigger_targets(SpawnedAcube, id);

            //let mut pl = player.single_mut().unwrap();
            //pl.n_cube += 1;
            //println!("n cube: {}", pl.n_cube);
        }
    }
}

#[derive(Event)]
struct ReadyToConnect;

pub fn join_cube(
    trig: Trigger<Pointer<Click>>,
    mut selected_pair_entity: Single<&mut PairEntitySelection, With<Player>>,
    mut commands: Commands,
) {
    let selected_entity = trig.target();
    let (ent1, ent2) = selected_pair_entity.0;
    let mut ents = [ent1, ent2];
    if let Some((n, _)) = ents
        .into_iter()
        .enumerate()
        .find(|(_n, x)| matches!(x, None))
    {
        ents[n] = Some(selected_entity);
        dbg!(&ents);
        selected_pair_entity.0 = (ents[0], ents[1]);
    } else {
        commands.trigger(ReadyToConnect);
    }
}

fn connect_entities(
    _trig: Trigger<ReadyToConnect>,
    mut selected_entities: Query<&mut PairEntitySelection, With<Player>>,
    mut commands: Commands,
) {
    let mut pair_entity = selected_entities.single_mut().unwrap();
    let (ent1, ent2) = pair_entity.0;
    let ent1 = ent1.unwrap();
    let ent2 = ent2.unwrap();
    let joint = RopeJointBuilder::new(80.0)
        .local_anchor1(Vec2 { x: 0.0, y: 0.0 })
        .local_anchor2(Vec2 { x: 0.0, y: 0.0 });
    commands.entity(ent1).insert(ImpulseJoint::new(ent2, joint));

    *pair_entity = PairEntitySelection((None, None));
}
