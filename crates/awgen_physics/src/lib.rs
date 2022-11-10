//! The core physics implementation for the Awgen game engine.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


pub mod position;
pub mod time;
pub mod velocity;

/// A re-export of all components and systems defined within this crate.
pub mod prelude {
    pub use super::position::*;
    pub use super::time::*;
    pub use super::velocity::*;
    pub use super::*;
}

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use prelude::*;
use std::time::Duration;


/// The implementation of the Awgen physics plugin. Handles collision, physics
/// frames, movement vectors, and similar forces that are applied to entities.
pub struct PhysicsPlugin {
    /// The number of physics frames per second.
    tickrate: f32,
}

impl PhysicsPlugin {
    /// Creates a new Physics plugin instance with the given physics tickrate.
    pub fn new(tickrate: f32) -> Self {
        Self {
            tickrate,
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        let frame_nanos = (1.0f64 / self.tickrate as f64) * 1_000_000_000.0f64;

        app.register_type::<Position>()
            .register_type::<PreviousPosition>()
            .register_type::<VelocitySource>()
            .register_type::<Movable>()
            .insert_resource(PhysicsTickrate::new(self.tickrate))
            .insert_resource(PhysicsFrame::default())
            .add_fixed_timestep(Duration::from_nanos(frame_nanos as u64), "tick")
            .add_fixed_timestep_child_stage("tick")
            .add_fixed_timestep_system("tick", 0, push_position_stack)
            .add_fixed_timestep_system("tick", 1, apply_velocity)
            .add_system(update_physics_frame)
            .add_system(update_render_position.after(update_physics_frame));
    }
}
