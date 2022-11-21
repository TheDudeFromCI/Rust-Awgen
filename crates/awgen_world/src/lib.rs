//! The voxel world data structure for Awgen.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


pub mod iterators;
pub mod populator;
pub mod world;


/// A re-export of all components and systems defined within this crate.
pub mod prelude {
    pub use super::populator::*;
    pub use super::world::*;
    pub use super::*;
}


use bevy::prelude::*;
use prelude::*;
use std::marker::PhantomData;


/// The world data structure plugin implementation.
///
/// This should be used in conjunction with [WorldDataTypePlugin] for
/// implementing each required block data type.
#[derive(Debug, Clone, Default)]
pub struct WorldDataPlugin;

impl Plugin for WorldDataPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChunkAnchor>()
            .register_type::<VoxelChunkStates>()
            .add_event::<LoadChunkEvent>()
            .add_system(load_chunks);
    }
}


/// A mini extension plugin for the WorldDataPlugin that registers all relevant
/// systems and components for a specific block data type.
#[derive(Debug, Clone, Default)]
pub struct WorldDataTypePlugin<BlockData>
where BlockData: Default + Copy + Send + Sync + 'static {
    /// To allow for the existence of the BlockData generic.
    _data: PhantomData<BlockData>,
}

impl<BlockData> Plugin for WorldDataTypePlugin<BlockData>
where BlockData: Default + Copy + Send + Sync + 'static
{
    fn build(&self, app: &mut App) {
        app.register_type::<VoxelWorld<BlockData>>();
    }
}
