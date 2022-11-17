//! The voxel world data structure for Awgen.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


pub mod world;


/// A re-export of all components and systems defined within this crate.
pub mod prelude {
    pub use super::world::*;
    pub use super::*;
}


use bevy::prelude::*;
use prelude::*;


/// The world data structure plugin implementation.
#[derive(Debug, Clone, Default)]
pub struct WorldDataPlugin;

impl Plugin for WorldDataPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VoxelWorld>();
    }
}
