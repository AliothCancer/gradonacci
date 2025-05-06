use bevy::prelude::*;

use crate::cube_chain::spawn_cube_chain;

pub struct MouseInteractionPlugin;

impl Plugin for MouseInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerMouseCoor>()
            .add_systems(Startup, spawn_cube_chain);
    }
}

#[derive(Resource)]
pub struct PlayerMouseCoor {
    pub x: f32,
    pub y: f32,
}
impl PlayerMouseCoor {
    pub fn update(&mut self, x: f32, y: f32) {
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

/// Add a cube to the the chain at click
pub fn add_child(mouse_coor: Res<PlayerMouseCoor>) {}
