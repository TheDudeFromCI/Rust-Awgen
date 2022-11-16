//! Contains prefabs related to spawning player and player-related entity
//! structures.


use awgen_client::prelude::{CameraController, MouseController, WasdController};
use awgen_physics::InterpolatedRigidBodyBundle;
use bevy::prelude::*;


/// A system command to spawn a new player instance.
pub fn spawn_player(mut commands: Commands) {
    let camera = commands
        .spawn((Name::new("Camera"), Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.85, 0.0),
            ..default()
        }))
        .id();

    let player = commands
        .spawn((
            Name::new("Player"),
            InterpolatedRigidBodyBundle::default(),
            WasdController::default(),
            MouseController::default(),
        ))
        .add_child(camera)
        .id();

    commands.entity(player).insert(CameraController {
        camera: Some(camera),
    });
}
