//! Voxel components that allow for the storage of block data in 16x16x16 chunks
//! for easy access.
//!
//! This module also contains the VoxelReader and VoxelWriter helper structs for
//! easier manipulation of voxel worlds across many chunks.


use anyhow::{anyhow, Result};
use arrayvec::ArrayVec;
use bevy::ecs::query::QueryIter;
use bevy::prelude::*;


/// Reads the block data from the requested block position and world from within
/// the given chunk query.
///
/// If the requested chunk is not within the chunk query, then None is returned.
fn get_block_data<'scope, 'world, 'state, 'cmd_scope, BlockData>(
    world: &'scope Entity,
    chunks: QueryIter<'world, 'state, &'cmd_scope VoxelChunk<BlockData>, ()>,
    block_pos: IVec3,
) -> Option<&'cmd_scope BlockData>
where
    'scope: 'cmd_scope,
    'world: 'cmd_scope,
    'state: 'cmd_scope,
    BlockData: Default + Sync + Send + 'static,
{
    let chunk_coords = block_pos >> 4;
    let local_pos = block_pos & 15;

    chunks
        .filter(|chunk| chunk.world().eq(&Some(*world)))
        .find(|chunk| chunk.coords().eq(&chunk_coords))
        .map(|chunk| chunk.get(local_pos))
}


/// Writes the block data to the requested block position and world from within
/// the given chunk query.
///
/// If the requested chunk is not within the chunk query, then an error is
/// returned.
fn set_block_data<'scope, 'world, 'state, 'cmd_scope, BlockData>(
    world: &'scope Entity,
    chunks: QueryIter<'world, 'state, &'cmd_scope mut VoxelChunk<BlockData>, ()>,
    block_pos: IVec3,
    data: BlockData,
) -> Result<()>
where
    'scope: 'cmd_scope,
    'world: 'cmd_scope,
    'state: 'cmd_scope,
    BlockData: Default + Sync + Send + 'static,
{
    let chunk_coords = block_pos >> 4;
    let local_pos = block_pos & 15;

    let chunk = chunks
        .filter(|chunk| chunk.world.eq(&Some(*world)))
        .find(|chunk| chunk.coords().eq(&chunk_coords));

    if let Some(mut chunk) = chunk {
        chunk.set(local_pos, data);
        Ok(())
    } else {
        Err(anyhow!("Chunk does not exist!"))
    }
}


/// A command helper struct for reading block data across multiple chunks within
/// a world.
pub struct VoxelReader<'world, 'state, 'cmd_scope, BlockData>
where BlockData: Default + Sync + Send + 'static {
    /// The world that is being accessed.
    world: Entity,

    /// The query of chunks to read from and their corresponding block data.
    chunks: Query<'world, 'state, &'cmd_scope VoxelChunk<BlockData>>,
}

impl<'world, 'state, 'cmd_scope, BlockData> VoxelReader<'world, 'state, 'cmd_scope, BlockData>
where BlockData: Default + Sync + Send + 'static
{
    /// Creates a new VoxelReader instance for the given world entity and chunk
    /// query.
    pub fn from(
        world: Entity,
        chunks: Query<'world, 'state, &'cmd_scope VoxelChunk<BlockData>>,
    ) -> Self {
        Self {
            world,
            chunks,
        }
    }

    /// Gets the block data of the requested type at the given block position
    /// within this world.
    pub fn get<'scope>(&'scope self, block_pos: IVec3) -> Option<&'cmd_scope BlockData>
    where
        'scope: 'cmd_scope,
        'world: 'cmd_scope,
        'state: 'cmd_scope, {
        get_block_data(&self.world, self.chunks.iter(), block_pos)
    }
}


/// A command helper struct for reading and writing block data across multiple
/// chunks within a world.
pub struct VoxelWriter<'world, 'state, 'cmd_scope, BlockData>
where BlockData: Default + Sync + Send + 'static {
    /// The world that is being accessed.
    world: Entity,

    /// The query of chunks to write to and their corresponding block data.
    chunks: Query<'world, 'state, &'cmd_scope mut VoxelChunk<BlockData>>,
}

impl<'world, 'state, 'cmd_scope, BlockData> VoxelWriter<'world, 'state, 'cmd_scope, BlockData>
where BlockData: Default + Sync + Send + 'static
{
    /// Creates a new VoxelWriter instance for the given world entity and chunk
    /// query.
    pub fn from(
        world: Entity,
        chunks: Query<'world, 'state, &'cmd_scope mut VoxelChunk<BlockData>>,
    ) -> Self {
        Self {
            world,
            chunks,
        }
    }

    /// Gets the block data of the requested type at the given block position
    /// within this world.
    pub fn get<'scope>(&'scope self, block_pos: IVec3) -> Option<&'cmd_scope BlockData>
    where
        'scope: 'cmd_scope,
        'world: 'cmd_scope,
        'state: 'cmd_scope, {
        get_block_data(&self.world, self.chunks.iter(), block_pos)
    }


    /// Sets the block data stored at a specific block position within this
    /// world.
    pub fn set<'scope>(&'scope mut self, block_pos: IVec3, data: BlockData) -> Result<()>
    where
        'scope: 'cmd_scope,
        'world: 'cmd_scope,
        'state: 'cmd_scope, {
        set_block_data(&self.world, self.chunks.iter_mut(), block_pos, data)
    }
}


/// A marker component indicating the parent entity of a voxel world.
#[derive(Debug, Clone, Reflect, Component)]
pub struct VoxelWorld;


/// A single 16x16x16 grid of data values that are stored within a voxel chunk.
/// The block data is stored in a fixed array on the heap.
///
/// It is worth noting that due to the flexible nature of the generics attached
/// to this component, it is not registered by default Bevy. As a result, each
/// block data type that is used must manually be registered for VoxelChunks in
/// order to properly work with Bevy Reflect.
#[derive(Debug, Clone, Reflect, Component, Default)]
pub struct VoxelChunk<BlockData>
where BlockData: Default + Sync + Send + 'static {
    /// The block data stored within this chunk.
    #[reflect(ignore)]
    blocks: ArrayVec<BlockData, 4096>,

    /// The coordinate location of this chunk.
    chunk_coords: IVec3,

    /// The world that this chunk belongs to.
    world: Option<Entity>,
}

impl<BlockData> VoxelChunk<BlockData>
where BlockData: Default + Sync + Send + 'static
{
    /// Creates a new voxel chunk with the given chunk coordinates and world
    /// entity ID.
    pub fn new(coords: IVec3, world: Option<Entity>) -> Self {
        Self {
            blocks: default(),
            chunk_coords: coords,
            world,
        }
    }


    /// Gets the block data at the given local block coordinate.
    pub fn get(&self, pos: IVec3) -> &BlockData {
        let index = pos.x * 16 * 16 + pos.y * 16 + pos.z;
        &self.blocks[index as usize]
    }


    /// Sets the block data at the given local block coordinate.
    pub fn set(&mut self, pos: IVec3, data: BlockData) {
        let index = pos.x * 16 * 16 + pos.y * 16 + pos.z;
        self.blocks[index as usize] = data;
    }


    /// Gets the coordinate location of this chunk.
    pub fn coords(&self) -> IVec3 {
        self.chunk_coords
    }


    /// Gets the world that this chunk belongs to.
    pub fn world(&self) -> Option<Entity> {
        self.world
    }
}
