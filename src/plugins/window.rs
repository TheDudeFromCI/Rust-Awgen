//! The client-side rendering plugin. This plugin loads all default systems and
//! plugins provided by Bevy to introduce window handling, rendering, asset
//! loading, animations, and so on.
//!
//! In addition, this plugin provides some basic overrides to the default plugin
//! list in order to further customize the setup for Awgen's engine needs.


use bevy::prelude::*;


/// The default window title for the Awgen game engine.
const WINDOW_TITLE: &str = "Awgen";


/// The default background clear color.
const CLEAR_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);


/// The window plugin implementation.
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR))
            .insert_resource(WindowDescriptor {
                title: WINDOW_TITLE.to_string(),
                ..default()
            })
            .add_plugins(DefaultPlugins);
    }
}
