use bevy::{color::palettes::basic::*, prelude::*};

use crate::player::player_plugin::{ClickMode, Player};

use super::*;

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut player: Single<&mut Player>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = match player.0 {
                    crate::player::player_plugin::ClickMode::SpawnCube => {
                        player.0 = ClickMode::JoinCube;
                        "JoinCube".to_string()
                    }
                    crate::player::player_plugin::ClickMode::JoinCube => {
                        player.0 = ClickMode::SpawnCube;
                        "SpawnCube".to_string()
                    }
                };

                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
