use bevy::prelude::*;

use super::NORMAL_BUTTON;

pub fn button(text: String, asset_server: &AssetServer) -> impl Bundle + use<> {
    (
        Button,
        Node {
            width: Val::Px(300.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(3.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load("fonts/MonofurNerdFont-Regular.ttf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}
