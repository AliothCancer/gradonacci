use bevy::prelude::*;

use crate::cube_chain::spawn_cube_chain;

use super::resources::PlayerMouseCoor;

pub struct MouseInteractionPlugin;

impl Plugin for MouseInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerMouseCoor>()
            .add_systems(Startup, spawn_cube_chain);
    }
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
