//! The voxel world mesh generation handler for Awgen.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


/// A re-export of all components and systems defined within this crate.
pub mod prelude {
    pub use super::*;
}


use bevy::prelude::*;


/// The world mesh plugin implementation.
#[derive(Debug, Clone)]
pub struct WorldMeshPlugin;

impl Plugin for WorldMeshPlugin {
    fn build(&self, _app: &mut App) {}
}
