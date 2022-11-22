//! Contains the chunk mesh generation functionality.


use crate::prelude::{BlockOcclusion, BlockShape};
use awgen_math::region::Region;
use awgen_world::world::VoxelWorld;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;


/// A wrapper for containing temporary mesh data to be converted into a proper
/// mesh later.
#[derive(Debug, Clone, Default)]
pub struct ChunkMesher {
    /// The list of indices in this mesh.
    pub indices: Vec<u16>,

    /// The list of vertices in this mesh.
    pub vertices: Vec<[f32; 3]>,

    /// The list of normals in this mesh.
    pub normals: Vec<[f32; 3]>,

    /// The list of uvs in this mesh.
    pub uvs: Vec<[f32; 2]>,
}

impl From<ChunkMesher> for Mesh {
    fn from(val: ChunkMesher) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, val.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, val.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, val.uvs);
        mesh.set_indices(Some(Indices::U16(val.indices)));
        mesh
    }
}


/// Generates a new chunk mesh from the given voxel reader for the chunk at the
/// indicates chunk coordinates.
pub fn generate_chunk_mesh(chunk_coords: IVec3, shapes: VoxelWorld<BlockShape>) -> Mesh {
    let mut mesher = ChunkMesher::default();

    let region = Region::from_size((chunk_coords << 4) - 1, IVec3::new(18, 18, 18));
    let shape_data = shapes.get_block_region(region);

    for pos in Region::CHUNK.iter() {
        let block_index = region.point_to_index(pos).unwrap();

        if shape_data[block_index].get_occlusion().contains(BlockOcclusion::INNER) {
            continue;
        }

        let check_dir = |offset, flag: BlockOcclusion, occlusion: &mut BlockOcclusion| {
            let index = region.point_to_index(pos + offset).unwrap();
            let shape = shape_data[index];
            if shape.get_occlusion().contains(flag.opposite_face()) {
                occlusion.insert(flag);
            }
        };

        let mut occlusion = BlockOcclusion::empty();
        #[rustfmt::skip]
        {
            check_dir(IVec3::NEG_X, BlockOcclusion::NEG_X, &mut occlusion);
            check_dir(IVec3::    X, BlockOcclusion::POS_X, &mut occlusion);
            check_dir(IVec3::NEG_Y, BlockOcclusion::NEG_Y, &mut occlusion);
            check_dir(IVec3::    Y, BlockOcclusion::POS_Y, &mut occlusion);
            check_dir(IVec3::NEG_Z, BlockOcclusion::NEG_Z, &mut occlusion);
            check_dir(IVec3::    Z, BlockOcclusion::POS_Z, &mut occlusion);
        }

        shape_data[block_index].push_to_mesh(&mut mesher, &occlusion, pos.as_vec3());
    }

    mesher.into()
}
