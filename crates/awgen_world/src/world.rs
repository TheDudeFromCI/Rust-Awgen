//! Voxel components that allow for the storage of block data in 16x16x16 chunks
//! for easy access.
//!
//! This module also contains the VoxelReader and VoxelWriter helper structs for
//! easier manipulation of voxel worlds across many chunks.


use crate::iterators::CuboidIterator;
use anyhow::Result;
use bevy::prelude::*;


/// A single 16x16x16 grid of data values that are stored within a voxel chunk.
/// The block data is stored in a fixed array on the heap.
#[derive(Debug)]
struct VoxelChunk<BlockData>
where BlockData: Default + Copy + Send + Sync + 'static {
    /// The block data array for this chunk.
    blocks: Box<[BlockData; 4096]>,
}

impl<BlockData> Default for VoxelChunk<BlockData>
where BlockData: Default + Copy + Send + Sync + 'static
{
    fn default() -> Self {
        Self {
            blocks: Box::new([default(); 4096]),
        }
    }
}


/// A single 16x16x16 grid of chunks within a voxel world that store a single,
/// specific type of data. These chunks may optionally be defined.
#[derive(Debug)]
struct VoxelRegion<BlockData>
where BlockData: Default + Copy + Send + Sync + 'static {
    /// The chunk array grid for this region.
    chunks: Box<[Option<VoxelChunk<BlockData>>; 4096]>,

    /// The coordinates of this region.
    region_coords: IVec3,
}

impl<BlockData> VoxelRegion<BlockData>
where BlockData: Default + Copy + Send + Sync + 'static
{
    /// Creates a new, empty region instance at the given region coordinates.
    fn new(region_coords: IVec3) -> Self {
        Self {
            chunks: Box::new([(); 4096].map(|_| None)),
            region_coords,
        }
    }
}

/// A marker component indicating the parent entity of a voxel world.
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct VoxelWorld<BlockData>
where BlockData: Default + Copy + Send + Sync + 'static {
    /// A list of all chunk regions within this world.
    #[reflect(ignore)]
    regions: Vec<VoxelRegion<BlockData>>,
}

impl<BlockData> VoxelWorld<BlockData>
where BlockData: Default + Copy + Send + Sync + 'static
{
    /// Gets the block data at the given block position.
    ///
    /// If the block position is not within a loaded chunk, then the default
    /// value for the block data is returned.
    pub fn get_block_data(&self, block_pos: IVec3) -> BlockData {
        let region_coords = block_pos >> 8;

        let chunk_coords: IVec3 = (block_pos >> 4) & 15;
        let chunk_index = chunk_coords.x * 16 * 16 + chunk_coords.y * 16 + chunk_coords.z;

        let block_coords = block_pos & 15;
        let block_index = block_coords.x * 16 * 16 + block_coords.y * 16 + block_coords.z;

        self.regions
            .iter()
            .find(|r| r.region_coords.eq(&region_coords))
            .and_then(|r| r.chunks[chunk_index as usize].as_ref())
            .map_or_else(|| BlockData::default(), |c| c.blocks[block_index as usize])
    }


    /// Gets the data from a cuboid region of blocks all at once.
    ///
    /// For reading data from a large volume of blocks at a time, this method is
    /// much more efficient that reading data from a single block at a time, as
    /// the same chunks do not need to be searched out multiple times.
    ///
    /// The returned vector is a list of all block data values, where the block
    /// at location X, Y, Z within the local region coordinates is located
    /// at the index X * 256 + Y * 16 + Z within the returned vector list.
    /// If the region overlaps any unloaded or otherwise non-existent
    /// chunks, those locations are filled with the default value for the
    /// block data.
    ///
    /// The region to load is specified by the tuple (IVec3, IVec3), where each
    /// element is one opposite corner of the region, inclusive.
    pub fn get_block_region(&self, region: (IVec3, IVec3)) -> Vec<BlockData> {
        let min = region.0.min(region.1);
        let max = region.0.max(region.1);
        let size = max - min + 1;

        let block_count = size.x * size.y * size.z;
        let mut data = vec![BlockData::default(); block_count as usize];

        for chunk_coords in CuboidIterator::from_points(min >> 4, max >> 4) {
            let region_coords = chunk_coords >> 4;
            let chunk_index = (chunk_coords.x & 15) * 16 * 16
                + (chunk_coords.y & 15) * 16
                + (chunk_coords.z & 15);

            let chunk = self
                .regions
                .iter()
                .find(|r| r.region_coords.eq(&region_coords))
                .and_then(|r| r.chunks[chunk_index as usize].as_ref());

            let block_min = min.max(chunk_coords << 4);
            let block_max = max.min((chunk_coords << 4) + 15);

            for block in CuboidIterator::from_points(block_min, block_max) {
                let data_pos = block - min;
                let data_index = data_pos.x * size.y * size.z + data_pos.y * size.z + data_pos.z;

                if let Some(chunk) = chunk {
                    let index = (block.x & 15) * 16 * 16 + (block.y & 15) * 16 + (block.z & 15);
                    data[data_index as usize] = chunk.blocks[index as usize];
                } else {
                    data[data_index as usize] = BlockData::default();
                }
            }
        }

        data
    }


    /// Sets the block data at the given block position.
    ///
    /// If the block position is located within an unloaded chunk, a new chunk
    /// created at that location with all default values and the data value
    /// is written to it.
    pub fn set_block_data(&mut self, block_pos: IVec3, data: BlockData) {
        let region_coords = block_pos >> 8;

        let chunk_coords: IVec3 = (block_pos >> 4) & 15;
        let chunk_index = chunk_coords.x * 16 * 16 + chunk_coords.y * 16 + chunk_coords.z;

        let block_coords = block_pos & 15;
        let block_index = block_coords.x * 16 * 16 + block_coords.y * 16 + block_coords.z;

        for region in &mut self.regions {
            if !region.region_coords.eq(&region_coords) {
                continue;
            }

            if let Some(chunk) = &mut region.chunks[chunk_index as usize] {
                chunk.blocks[block_index as usize] = data;
            } else {
                let mut chunk = VoxelChunk::<BlockData>::default();
                chunk.blocks[block_index as usize] = data;
                region.chunks[chunk_index as usize] = Some(chunk);
            }

            return;
        }

        let mut region = VoxelRegion::<BlockData>::new(region_coords);
        let mut chunk = VoxelChunk::<BlockData>::default();
        chunk.blocks[block_index as usize] = data;
        region.chunks[chunk_index as usize] = Some(chunk);
        self.regions.push(region);
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;


    #[test]
    fn read_write_world() {
        let mut world = VoxelWorld::<u8>::default();
        let pos = IVec3::new(15, 128, -3);

        assert_eq!(world.get_block_data(pos), 0);

        world.set_block_data(pos, 7);
        assert_eq!(world.get_block_data(pos), 7);
    }


    #[test]
    fn get_block_region() {
        let mut world = VoxelWorld::<u8>::default();
        world.set_block_data(IVec3::new(-1, 11, 4), 3);
        world.set_block_data(IVec3::new(-1, 11, 5), 3);

        let region = (IVec3::new(-3, 12, 2), IVec3::new(0, 10, 5));
        let data = world.get_block_region(region);

        assert_eq!(data.len(), 4 * 3 * 4);
        assert_eq!(data.iter().filter(|v| **v == 3).count(), 2);
    }
}
