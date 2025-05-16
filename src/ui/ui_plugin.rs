use bevy::prelude::*;

use super::{bundles::button, systems::button_system};

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, button_system);
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(button("SpawnCube".to_string(), &assets));
}
