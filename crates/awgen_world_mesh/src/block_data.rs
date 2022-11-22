//! Contains the block data definitions for world visual mesh generation and
//! collision mesh generation.


use crate::prelude::ChunkMesher;
use bevy::prelude::*;
use bitflags::bitflags;


bitflags! {
    /// Defines which sides of a block are occluded via a bitwise enum flag.
    ///
    /// If the flag is enabled, that side of the block is hidden. If false, that
    /// block face is visible.
    pub struct BlockOcclusion: u8 {
        /// The inside of the block itself. If true, then the block is hiding itself.
        const INNER = 0b00000001;

        /// The side of the block on the positive X axis.
        const POS_X = 0b00000010;

        /// The side of the block on the negative X axis.
        const NEG_X = 0b00000100;

        /// The side of the block on the positive Y axis.
        const POS_Y = 0b00001000;

        /// The side of the block on the negative Y axis.
        const NEG_Y = 0b00010000;

        /// The side of the block on the positive Z axis.
        const POS_Z = 0b00100000;

        /// The side of the block on the negative  axis.
        const NEG_Z = 0b01000000;
    }
}


impl BlockOcclusion {
    /// Gets the opposite facing value for this block occlusion.
    ///
    /// For a positive value along an axis, this function will return the
    /// negative value of that axis. Likewise, negative value will return
    /// the positive counter parts. For inner block occlusion, the value remains
    /// unchanged.
    ///
    /// This effect is applied for all defined directional values.
    pub fn opposite_face(&self) -> Self {
        let mut value = BlockOcclusion::empty();

        if self.contains(BlockOcclusion::INNER) {
            value |= BlockOcclusion::INNER;
        }

        if self.contains(BlockOcclusion::POS_X) {
            value |= BlockOcclusion::NEG_X;
        }

        if self.contains(BlockOcclusion::NEG_X) {
            value |= BlockOcclusion::POS_X;
        }

        if self.contains(BlockOcclusion::POS_Y) {
            value |= BlockOcclusion::NEG_Y;
        }

        if self.contains(BlockOcclusion::NEG_Y) {
            value |= BlockOcclusion::POS_Y;
        }

        if self.contains(BlockOcclusion::POS_Z) {
            value |= BlockOcclusion::NEG_Z;
        }

        if self.contains(BlockOcclusion::NEG_Z) {
            value |= BlockOcclusion::POS_Z;
        }

        value
    }
}


/// The block shape to use when generating a chunk mesh.
#[derive(Debug, Clone, Copy, Reflect, Default, PartialEq, Eq)]
pub enum BlockShape {
    /// This block is an empty block and contains no visual or collision mesh
    /// elements.
    #[default]
    Empty,

    /// A basic one meter cube shape.
    Cube,

    /// Allows for a custom block model that is not defined from within the
    /// chunk mesh, but instead. a separate model that is handled by another
    /// entity.
    Custom,
}

impl BlockShape {
    /// Gets the block occlusion flags for this block shape.
    ///
    /// If the element within the flag is true, then that face is occluded by
    /// this block shape.
    pub fn get_occlusion(&self) -> BlockOcclusion {
        match self {
            BlockShape::Empty => BlockOcclusion::INNER,
            BlockShape::Custom => BlockOcclusion::empty(),

            BlockShape::Cube => {
                BlockOcclusion::NEG_X
                    | BlockOcclusion::POS_X
                    | BlockOcclusion::NEG_Y
                    | BlockOcclusion::POS_Y
                    | BlockOcclusion::NEG_Z
                    | BlockOcclusion::POS_Z
            },
        }
    }


    /// Writes the mesh data for this block shape to the temporary mesh, based
    /// on the provided block occlusion specifications.
    pub fn push_to_mesh(&self, mesh: &mut ChunkMesher, occlusion: &BlockOcclusion, pos: Vec3) {
        match self {
            BlockShape::Empty | BlockShape::Custom => {},
            BlockShape::Cube => write_cube(mesh, occlusion, pos),
        }
    }
}


/// Writes a cube shape to the temporary mesh.
fn write_cube(mesh: &mut ChunkMesher, occlusion: &BlockOcclusion, pos: Vec3) {
    /// A lookup table for the vertex positions of a cube.
    const VERTS: [Vec3; 8] = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ];

    let mut quad = |v0, v1, v2, v3, normal: Vec3| {
        let vert_count = mesh.vertices.len() as u16;
        mesh.indices.push(vert_count);
        mesh.indices.push(vert_count + 1);
        mesh.indices.push(vert_count + 2);
        mesh.indices.push(vert_count);
        mesh.indices.push(vert_count + 2);
        mesh.indices.push(vert_count + 3);

        mesh.vertices.push((pos + VERTS[v0] as Vec3).into());
        mesh.vertices.push((pos + VERTS[v1] as Vec3).into());
        mesh.vertices.push((pos + VERTS[v2] as Vec3).into());
        mesh.vertices.push((pos + VERTS[v3] as Vec3).into());

        mesh.normals.push(normal.into());
        mesh.normals.push(normal.into());
        mesh.normals.push(normal.into());
        mesh.normals.push(normal.into());

        mesh.uvs.push([0.0, 0.0]);
        mesh.uvs.push([0.0, 1.0]);
        mesh.uvs.push([1.0, 1.0]);
        mesh.uvs.push([1.0, 0.0]);
    };

    if !occlusion.contains(BlockOcclusion::NEG_X) {
        quad(0, 1, 3, 2, Vec3::NEG_X);
    }

    if !occlusion.contains(BlockOcclusion::POS_X) {
        quad(4, 6, 7, 5, Vec3::X);
    }

    if !occlusion.contains(BlockOcclusion::NEG_Y) {
        quad(0, 4, 5, 1, Vec3::NEG_Y);
    }

    if !occlusion.contains(BlockOcclusion::POS_Y) {
        quad(2, 3, 7, 6, Vec3::Y);
    }

    if !occlusion.contains(BlockOcclusion::NEG_Z) {
        quad(0, 2, 6, 4, Vec3::NEG_Z);
    }

    if !occlusion.contains(BlockOcclusion::POS_Z) {
        quad(1, 5, 7, 3, Vec3::Z);
    }
}
