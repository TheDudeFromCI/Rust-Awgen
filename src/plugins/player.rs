//! The main player input and controller plugin for manipulating the player
//! entity instance.


use crate::components::*;
use crate::systems::physics::*;
use crate::systems::player_input::*;
use bevy::prelude::*;


/// The implementation for the player plugin.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FirstPersonController>()
            .register_type::<WasdController>()
            .add_system(update_camera_rotation.before(copy_transform_from_look_dir))
            .add_system(wasd_movement.before(apply_movement))
            .add_system(toggle_cursor);
    }
}
