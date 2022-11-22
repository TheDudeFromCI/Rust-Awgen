//! The voxel world mesh generation handler for Awgen.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]
#![feature(stmt_expr_attributes)]


pub mod block_data;
pub mod mesher;


/// A re-export of all components and systems defined within this crate.
pub mod prelude {
    pub use super::block_data::*;
    pub use super::mesher::*;
    pub use super::*;
}


use bevy::prelude::*;


/// The world mesh plugin implementation.
#[derive(Debug, Clone, Default)]
pub struct WorldMeshPlugin;

impl Plugin for WorldMeshPlugin {
    fn build(&self, _app: &mut App) {}
}
