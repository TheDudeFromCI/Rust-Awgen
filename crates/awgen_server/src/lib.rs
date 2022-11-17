//! The server implementation for Awgen. Handles resource and world management
//! and distribution.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


/// A re-export of all components and systems defined within this crate.
pub mod prelude {
    pub use super::*;
}


use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::prelude::*;


/// The Awgen server plugin implementation.
#[derive(Debug, Clone, Default)]
pub struct ServerPlugin {
    /// Whether or not this plugin is loaded in debug mode.
    debug: bool,
}

impl ServerPlugin {
    /// Creates a new server plugin instance in debug mode.
    pub fn debug() -> Self {
        Self {
            debug: true,
        }
    }


    /// Gets whether or not this server is loaded in debug mode.
    pub fn is_debug(&self) -> bool {
        self.debug
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        if self.is_debug() {
            app.insert_resource(ReportExecutionOrderAmbiguities);
        }
    }
}
