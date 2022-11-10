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


use bevy::prelude::*;


/// The Awgen server plugin implementation.
#[derive(Default)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins);
    }
}
