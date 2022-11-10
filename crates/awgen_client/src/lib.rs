//! The client implementation for the Awgen game engine. Handles windows,
//! graphics, and user input.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


pub mod controller;


/// A re-export of all components and systems defined within this crate.
pub mod prelude {
    pub use super::controller::*;
    pub use super::*;
}


use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use prelude::*;


/// The default window title for the Awgen game engine.
const WINDOW_TITLE: &str = "Awgen";


/// The default background clear color.
const CLEAR_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);


/// The implementation for the Awgen client plugin. This plugin manages loading
/// the window, reading graphics, and handling user input. This plugin does not
/// handle any core game mechanics outside of basic player controls and systems.
#[derive(Debug, Clone, Default)]
pub struct ClientPlugin {
    /// Whether or not this plugin should be initialized in debug mode.
    debug: bool,
}

impl ClientPlugin {
    /// Creates a new debug-mode instance of the client plugin.
    pub fn debug() -> Self {
        Self {
            debug: true,
        }
    }


    /// Gets whether or not this Client plugin is initialized in debug mode.
    pub fn is_debug(&self) -> bool {
        self.debug
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let window_title = match self.is_debug() {
            true => WINDOW_TITLE.to_string(),
            false => format!("{WINDOW_TITLE} [Debug]"),
        };

        app.register_type::<WasdController>()
            .register_type::<MouseController>()
            .insert_resource(ClearColor(CLEAR_COLOR))
            .insert_resource(WindowDescriptor {
                title: window_title,
                ..default()
            })
            .add_plugins(DefaultPlugins)
            .add_system(wasd_velocity_input)
            .add_system(mouse_rotation_input)
            .add_system(toggle_cursor);

        if self.is_debug() {
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}
