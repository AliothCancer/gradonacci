use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::player_plugin::join_cube;

/// A plugin which automatically attach a bevy_rapier2d Mesh2d and MeshMaterial2d matching the shape
/// using an observer which wait for OnInsert of a crate::Shape component
pub struct RobotConstructorPlugin;
impl Plugin for RobotConstructorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_insert_shape_attach_mesh_and_material)
            .add_observer(
                |trigger: Trigger<OnAdd, EntityColor>,
                 color: Query<&EntityColor>,
                 mut commands: Commands,
                 mut materials: ResMut<Assets<ColorMaterial>>| {
                    let id = trigger.target();
                    let color = color.get(id).unwrap().0;
                    commands.entity(id).insert(MeshMaterial2d(
                        materials.add(ColorMaterial::from_color(color)),
                    ));
                },
            );
    }
}

#[derive(Component, Clone, Copy)]
pub enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

pub fn on_insert_shape_attach_mesh_and_material(
    trigger: Trigger<OnInsert, Shape>,
    shape: Query<&Shape>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let entity = trigger.target();
    let Ok(shape) = shape.get(entity) else {
        return;
    };

    let mesh_bundle = match *shape {
        Shape::Circle { radius } => (
            Mesh2d(meshes.add(Circle::new(radius))),
            Collider::ball(radius),
        ),
        Shape::Rectangle { width, height } => (
            Mesh2d(meshes.add(Rectangle::new(width * 2.0, height * 2.0))), // multiplication by 2.0 is necessary in bevy_rapier
            // since it takes half measurement, avian instead use whole measurment
            Collider::cuboid(width, height),
        ),
    };
    commands.entity(entity).insert((mesh_bundle,));
}

pub fn spawn_robot(commands: &mut Commands) -> Entity {
    let (height, width) = (40.0, 30.0);
    let ball_radius = 60.0;

    let head_bundle = robot_head(height, width);
    let ball_bundle = robot_ball(ball_radius);

    let joint = RopeJointBuilder::new(30.0)
        .local_anchor1(Vect::new(0.0, -ball_radius*2.0)) // ball local coord
        .local_anchor2(Vect::new(0.0, height/2.0)) // head body local coord
        ;
    let child = commands.spawn(ball_bundle).observe(join_cube).id();
    let parent = commands.spawn(head_bundle).observe(join_cube).id();
    commands
        .entity(child)
        .insert(ImpulseJoint::new(parent, joint));
    parent
}
#[derive(Component)]
pub struct EntityColor(pub Color);

pub fn robot_head(height: f32, width: f32) -> impl Bundle {
    (
        Robot::Head,
        Shape::Rectangle { width, height },
        RigidBody::Dynamic,
        EntityColor(Color::linear_rgb(0.1, 1.0, 0.3)),
        Velocity::zero(),
        GravityScale(-3.0),
        Transform::from_xyz(0.0, 200.0, 0.0),
    )
}
pub fn robot_ball(ball_radius: f32) -> impl Bundle {
    (
        Robot::Ball,
        RigidBody::Dynamic,
        AdditionalMassProperties::Mass(3000.0),
        EntityColor(Color::linear_rgb(0.0, 0.5, 1.0)),
        Shape::Circle {
            radius: ball_radius,
        },
        //Velocity::zero(),
        Transform::from_xyz(0.0, 100.0, 0.0),
    )
}

#[derive(Component)]
enum Robot {
    Head,
    Ball,
}
