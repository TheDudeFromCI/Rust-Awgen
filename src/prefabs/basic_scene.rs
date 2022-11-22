//! A temporary example scene.


use awgen_math::region::Region;
use awgen_world::world::VoxelWorld;
use awgen_world_mesh::prelude::{generate_chunk_mesh, BlockShape};
use bevy::prelude::*;


/// Spawns a 3D plane
pub fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        ..default()
    });

    let mut voxel_world = VoxelWorld::<BlockShape>::default();
    for pos in Region::from_points(IVec3::new(0, 0, 0), IVec3::new(15, 0, 15)).iter() {
        voxel_world.set_block_data(pos, BlockShape::Cube);
    }

    let mesh = generate_chunk_mesh(IVec3::ZERO, voxel_world);
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}
