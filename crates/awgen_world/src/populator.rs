//! This module aids in chunk population (either via a world generator or chunk
//! loading task) and chunk pruning (via chunk unloading).


use awgen_math::region::Region;
use awgen_physics::prelude::Position;
use bevy::prelude::*;


/// Defines an anchor within a world that forces a radius of chunks around
/// itself to stay loaded.
///
/// This component relies on the Position component in order to function.
#[derive(Debug, Clone, Reflect, Component, Default)]
#[reflect(Component)]
pub struct ChunkAnchor {
    /// The world that this chunk anchor is pinned to.
    pub world: Option<Entity>,

    /// The radius, in chunks, that are triggered to load around the chunk
    /// anchor.
    ///
    /// A value of 0 will only trigger a single chunk to remain loaded.
    pub radius: u16,

    /// The maximum number of chunks around this anchor that are allowed to
    /// remain loaded before being considered out of range.
    ///
    /// A value of 0 will only allow for a single chunk to be considered within
    /// range of this anchor.
    pub max_radius: u16,
}

impl ChunkAnchor {
    /// Creates a new chunk anchor instance.
    ///
    /// The world entity must be an entity that contains the VoxelChunkStates
    /// component.
    pub fn new(world: Entity, radius: u16, max_radius: u16) -> Self {
        Self {
            world: Some(world),
            radius,
            max_radius,
        }
    }
}


/// A handler for determining the chunk load states for a single voxel world.
#[derive(Debug, Clone, Reflect, Component, Default)]
#[reflect(Component)]
pub struct VoxelChunkStates {
    /// A list of chunk regions within the world.
    #[reflect(ignore)]
    regions: Vec<VoxelChunkStateRegion>,
}

impl VoxelChunkStates {
    /// Gets the ChunkState for the chunk at the indicated chunk coordinates.
    pub fn get_state(&self, chunk_coords: IVec3) -> ChunkState {
        let region_coords = chunk_coords >> 4;
        let index = Region::CHUNK.point_to_index(chunk_coords & 15).unwrap();

        self.regions
            .iter()
            .find(|r| r.region_coords.eq(&region_coords))
            .map_or(ChunkState::Unloaded, |r| r.chunks[index])
    }


    /// Changes the state of the chunk at the indicates chunk coordinates.
    pub fn set_state(&mut self, chunk_coords: IVec3, state: ChunkState) {
        let region_coords = chunk_coords >> 4;
        let index = Region::CHUNK.point_to_index(chunk_coords & 15).unwrap();

        if let Some((reg_index, region)) = self
            .regions
            .iter_mut()
            .enumerate()
            .find(|(_, r)| r.region_coords.eq(&region_coords))
        {
            region.chunks[index] = state;

            if region.chunks.iter().all(|c| *c == ChunkState::Unloaded) {
                self.regions.remove(reg_index);
            }
        } else if state != ChunkState::Unloaded {
            let mut region = VoxelChunkStateRegion::new(region_coords);
            region.chunks[index] = state;
            self.regions.push(region);
        }
    }
}


/// A 16x16x16 grid of chunk states for quick access.
#[derive(Debug, Clone)]
struct VoxelChunkStateRegion {
    /// The grid of chunk states within this region.
    chunks: Box<[ChunkState; 4096]>,

    /// The coordinates of this region.
    region_coords: IVec3,
}

impl VoxelChunkStateRegion {
    /// Creates a new chunk state region for the given region coordinates.
    fn new(region_coords: IVec3) -> Self {
        Self {
            chunks: Box::new([ChunkState::Unloaded; 4096]),
            region_coords,
        }
    }
}


/// A chunk loading state indicator.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ChunkState {
    /// Indicates that the chunk is not currently loaded.
    #[default]
    Unloaded,

    /// Indicates that the chunk is being loaded, or generated, in a background
    /// task.
    Loading,

    /// Indicates that the chunk is already loaded and is ready to use.
    Loaded,

    /// Indicates that the chunk is being unloaded.
    Unloading,
}


/// An event that is triggered when a chunk is requested to be loaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadChunkEvent {
    /// The coordinates of the chunk to be loaded.
    pub chunk_coords: IVec3,

    /// The voxel world to load the chunk in.
    pub world: Entity,
}


/// An event that is triggered when a chunk is requested to be unloaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnloadChunkEvent {
    /// The coordinates of the chunk to be unloaded.
    pub chunk_coords: IVec3,

    /// The voxel world to prune the chunk from.
    pub world: Entity,
}


/// Loads chunks around all current world anchors.
pub fn load_chunks(
    mut states: Query<&mut VoxelChunkStates>,
    anchors: Query<(&ChunkAnchor, &Position)>,
    mut load_chunk_ev: EventWriter<LoadChunkEvent>,
) {
    for (anchor, pos) in anchors.iter() {
        if let Some(world) = anchor.world {
            let mut world_states = states.get_mut(world).unwrap();

            let pos = pos.translation.as_ivec3() >> 4;
            let min = pos - anchor.radius as i32;
            let max = pos + anchor.radius as i32;
            let region = Region::from_points(min, max);

            for chunk in region.iter() {
                let state = world_states.get_state(chunk);

                if state == ChunkState::Unloaded {
                    world_states.set_state(chunk, ChunkState::Loading);
                    load_chunk_ev.send(LoadChunkEvent {
                        chunk_coords: chunk,
                        world,
                    });
                }
            }
        }
    }
}


/// Unloads unused chunks based on current world anchors.
#[allow(unused)]
pub fn unload_chunks(
    mut states: Query<&mut VoxelChunkStates>,
    anchors: Query<(&ChunkAnchor, &Position)>,
    mut unload_chunk_ev: EventWriter<UnloadChunkEvent>,
) {
    todo!();
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;


    #[test]
    fn load_nearby() {
        let mut app = App::new();
        app.add_event::<LoadChunkEvent>();
        app.add_system(load_chunks);

        let voxel_world = app.world.spawn(VoxelChunkStates::default()).id();
        app.world.spawn((
            Position {
                translation: Vec3::new(44.0, 2.1, -4.7), // Chunk Coords: (2, 0, -1)
                ..default()
            },
            ChunkAnchor::new(voxel_world, 1, 2),
        ));

        app.update();

        let load_chunk_ev = app.world.resource::<Events<LoadChunkEvent>>();
        let mut load_chunk_reader = load_chunk_ev.get_reader();
        let mut iter = load_chunk_reader.iter(load_chunk_ev);

        let min = IVec3::new(1, -1, -2);
        let max = IVec3::new(3, 1, 0);
        let region = Region::from_points(min, max);
        for pos in region.iter() {
            assert_eq!(
                iter.next(),
                Some(&LoadChunkEvent {
                    chunk_coords: pos,
                    world:        voxel_world,
                })
            );
        }

        assert_eq!(iter.next(), None);
    }
}
