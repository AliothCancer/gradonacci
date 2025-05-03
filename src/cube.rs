use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Component)]
pub struct Cube{
    width: f32,
    height: f32
}

impl Cube{
    pub fn bundle(width: f32, height: f32) -> impl Bundle {
        (
            // physics
            RigidBody::Dynamic,
            TransformInterpolation,
            Collider::rectangle(width, height),
            LinearVelocity::ZERO,
            // initial position
            Transform::from_xyz(0.0, 0.0, 0.0)
        )
    }

}

pub fn spawn_cube(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands
){
    commands.spawn(
        Cube::bundle(30.0, 30.0)
    )
    .insert((
        // appeareance
        Mesh2d(meshes.add(Rectangle::new(30.0,30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(LinearRgba::rgb(0.0, 0.0, 1.0)))),
    ));
}

