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


use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use prelude::*;


/// The implementation for the Awgen client plugin. This plugin manages loading
/// the window, reading graphics, and handling user input. This plugin does not
/// handle any core game mechanics outside of basic player controls and systems.
#[derive(Debug, Clone, Default)]
pub struct ClientPlugin {
    /// Whether or not this plugin is loaded in debug mode.
    debug: bool,
}

impl ClientPlugin {
    /// Creates a new client plugin instance in debug mode.
    pub fn debug() -> Self {
        Self {
            debug: true,
        }
    }


    /// Gets whether or not this client is loaded in debug mode.
    pub fn is_debug(&self) -> bool {
        self.debug
    }
}


impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        if self.is_debug() {
            app.insert_resource(ReportExecutionOrderAmbiguities)
                .add_plugin(WorldInspectorPlugin::new());
        }

        app.register_type::<WasdController>()
            .register_type::<MouseController>()
            .register_type::<CameraController>()
            .add_system(wasd_velocity_input.in_ambiguity_set("player_controls"))
            .add_system(mouse_rotation_input.in_ambiguity_set("player_controls"))
            .add_system(toggle_cursor.in_ambiguity_set("player_controls"))
            .add_system(apply_camera_transform.after(mouse_rotation_input));
    }
}
