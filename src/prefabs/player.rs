//! Contains prefabs related to spawning player and player-related entity
//! structures.


use crate::components::{
    CopyFromLookDir, FirstPersonController, LookDirection, MovementInput, MovementSpeed, WasdController
};
use bevy::prelude::*;


/// A system command to spawn a new player instance.
pub fn spawn_player(mut commands: Commands) {
    let camera = commands
        .spawn()
        .insert(Name::new("Camera"))
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .id();

    let player = commands
        .spawn()
        .insert(Name::new("Player"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(LookDirection::default())
        .insert(MovementSpeed::default())
        .insert(MovementInput::from_look())
        .insert(FirstPersonController::default())
        .insert(WasdController::default())
        .add_child(camera)
        .id();

    commands.get_or_spawn(camera).insert(CopyFromLookDir::from(player));
}
