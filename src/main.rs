//! Awgen is a sandbox game with a heavy emphasis on also acting as a game engine to make smaller mini-games within.

#![warn(missing_docs)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Awgen".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(toggle_cursor)
        .run();
}

fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::F11) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}
