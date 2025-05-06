use bevy::prelude::*;

#[derive(Component)]
pub struct CubeChain;

pub fn spawn_cube_chain(mut commands: Commands) {
    commands.spawn(CubeChain);
}
