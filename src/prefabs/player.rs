//! Contains prefabs related to spawning player and player-related entity
//! structures.


use awgen_client::prelude::{CameraController, MouseController, WasdController};
use awgen_physics::InterpolatedRigidBodyBundle;
use bevy::prelude::*;


/// A system command to spawn a new player instance.
pub fn spawn_player(mut commands: Commands) {
    let camera = commands
        .spawn()
        .insert(Name::new("Camera"))
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.85, 0.0),
            ..default()
        })
        .id();

    let player = commands
        .spawn()
        .insert(Name::new("Player"))
        .insert_bundle(InterpolatedRigidBodyBundle::default())
        .insert(WasdController::default())
        .insert(MouseController::default())
        .add_child(camera)
        .id();

    commands.entity(player).insert(CameraController {
        camera: Some(camera),
    });
}
