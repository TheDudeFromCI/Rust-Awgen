//! Voxel components that allow for the storage of block data in 16x16x16 chunks
//! for easy access.
//!
//! This module also contains the VoxelReader and VoxelWriter helper structs for
//! easier manipulation of voxel worlds across many chunks.


use anyhow::{anyhow, Result};
use bevy::prelude::*;


/// Reads the block data from the requested block position and world from within
/// the given chunk query.
///
/// If the requested chunk is not within the chunk query, then None is returned.
pub fn get_block_data<'c, BlockData>(
    world: &Entity,
    chunks: &'c Query<&VoxelChunk<BlockData>>,
    block_pos: IVec3,
) -> Option<&'c BlockData>
where
    BlockData: Default + Copy + Sync + Send + 'static,
{
    let chunk_coords = block_pos >> 4;
    let local_pos = block_pos & 15;

    chunks
        .iter()
        .filter(|chunk| chunk.world().eq(world))
        .find(|chunk| chunk.coords().eq(&chunk_coords))
        .map(|chunk| chunk.get(local_pos))
}


/// Reads the block data from the requested block position and world from within
/// the given chunk query.
///
/// If the requested chunk is not within the chunk query, then None is returned.
///
/// This function is identical to [get_block_data], with the only difference
/// being that this function accepts a mutable chunk query instead of an
/// immutable one.
pub fn get_block_data_mut<'c, BlockData>(
    world: &Entity,
    chunks: &'c Query<&mut VoxelChunk<BlockData>>,
    block_pos: IVec3,
) -> Option<&'c BlockData>
where
    BlockData: Default + Copy + Sync + Send + 'static,
{
    let chunk_coords = block_pos >> 4;
    let local_pos = block_pos & 15;

    chunks
        .iter()
        .filter(|chunk| chunk.world().eq(world))
        .find(|chunk| chunk.coords().eq(&chunk_coords))
        .map(|chunk| chunk.get(local_pos))
}


/// Writes the block data to the requested block position and world from within
/// the given chunk query.
///
/// If the requested chunk is not within the chunk query, then an error is
/// returned.
pub fn set_block_data<BlockData>(
    world: &Entity,
    chunks: &mut Query<&mut VoxelChunk<BlockData>>,
    block_pos: IVec3,
    data: BlockData,
) -> Result<()>
where
    BlockData: Default + Copy + Sync + Send + 'static,
{
    let chunk_coords = block_pos >> 4;
    let local_pos = block_pos & 15;

    let chunk = chunks
        .iter_mut()
        .filter(|chunk| chunk.world.eq(world))
        .find(|chunk| chunk.coords().eq(&chunk_coords));

    if let Some(mut chunk) = chunk {
        chunk.set(local_pos, data);
        Ok(())
    } else {
        Err(anyhow!("Chunk does not exist!"))
    }
}


/// A marker component indicating the parent entity of a voxel world.
#[derive(Debug, Clone, Reflect, Component, Default)]
pub struct VoxelWorld;


/// A single 16x16x16 grid of data values that are stored within a voxel chunk.
/// The block data is stored in a fixed array on the heap.
///
/// It is worth noting that due to the flexible nature of the generics attached
/// to this component, it is not registered by default Bevy. As a result, each
/// block data type that is used must manually be registered for VoxelChunks in
/// order to properly work with Bevy Reflect.
#[derive(Debug, Clone, Reflect, Component)]
pub struct VoxelChunk<BlockData>
where BlockData: Default + Copy + Sync + Send + 'static {
    /// The block data stored within this chunk.
    #[reflect(ignore)]
    blocks: Box<[BlockData; 4096]>,

    /// The coordinate location of this chunk.
    chunk_coords: IVec3,

    /// The world that this chunk belongs to.
    world: Entity,
}

impl<BlockData> VoxelChunk<BlockData>
where BlockData: Default + Copy + Sync + Send + 'static
{
    /// Creates a new voxel chunk with the given chunk coordinates and world
    /// entity ID.
    pub fn new(coords: IVec3, world: Entity) -> Self {
        Self {
            blocks: Box::new([default(); 4096]),
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
    pub fn world(&self) -> Entity {
        self.world
    }
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn read_write_world() {
        let mut app = App::new();

        let world = app.world.spawn(VoxelWorld::default()).id();
        app.world.spawn(VoxelChunk::<u8>::new(IVec3::ZERO, world));

        fn voxel_system(
            worlds: Query<Entity, With<VoxelWorld>>,
            mut chunks: Query<&mut VoxelChunk<u8>>,
        ) {
            let data = 6;
            let block_pos = IVec3::new(1, 2, 3);

            for world in worlds.iter() {
                assert_eq!(*get_block_data_mut(&world, &chunks, block_pos).unwrap(), 0);
                set_block_data(&world, &mut chunks, block_pos, data).unwrap();
                assert_eq!(*get_block_data_mut(&world, &chunks, block_pos).unwrap(), 6);
            }
        }

        app.add_system(voxel_system);
        app.update();
    }
}
