//! A lightweight physics implementation in use for Awgen. Handles basic
//! collision handling, movement vectors, friction, and entity movement inputs.


use crate::components::*;
use crate::systems::physics::*;
use bevy::prelude::*;


/// The physics plugin implementation.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LookDirection>()
            .register_type::<MovementSpeed>()
            .register_type::<MovementInput>()
            .register_type::<CopyFromLookDir>()
            .add_system(copy_transform_from_look_dir)
            .add_system(apply_movement.after(copy_transform_from_look_dir));
    }
}
