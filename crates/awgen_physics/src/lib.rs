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
        let frame_nanos = ((1.0f64 / self.tickrate as f64) * 1_000_000_000.0f64) as u64;

        app.register_type::<Position>()
            .register_type::<PreviousPosition>()
            .register_type::<VelocitySource>()
            .register_type::<Movable>()
            .insert_resource(PhysicsTickrate::new(self.tickrate))
            .insert_resource(PhysicsFrame::default())
            .add_fixed_timestep_before_stage(
                CoreStage::Update,
                Duration::from_nanos(frame_nanos),
                "pre_tick",
            )
            .add_fixed_timestep_system("pre_tick", 0, push_position_stack)
            .add_fixed_timestep_system("pre_tick", 0, prepare_physics_render_frame)
            .add_fixed_timestep(Duration::from_nanos(frame_nanos), "tick")
            .add_fixed_timestep_after_stage(
                CoreStage::Update,
                Duration::from_nanos(frame_nanos),
                "post_tick",
            )
            .add_fixed_timestep_system("post_tick", 0, apply_velocity)
            .add_system_to_stage(CoreStage::PostUpdate, update_physics_render_frame)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_render_position.after(update_physics_render_frame),
            );
    }
}


/// A bundle for a movable rigid body object.
#[derive(Bundle, Default)]
pub struct RigidBodyBundle {
    /// The current position of the rigid body.
    position: Position,

    /// Marks this rigid body as capable of applying force to other objects.
    velocity_source: VelocitySource,

    /// Marks this rigid body as capable of being moved by other velocity
    /// sources.
    movable: Movable,
}


/// A bundle for a movable rigid body object that is interpolated between
/// frames. Contains all of the elements from a standard rigid body, as well as
/// a transform, and interpolation handlers.
#[derive(Bundle, Default)]
pub struct InterpolatedRigidBodyBundle {
    /// The current position of the rigid body.
    #[bundle]
    rigid_body: RigidBodyBundle,

    /// The position of the rigid body on the previous frame.
    previous_position: PreviousPosition,

    /// The transform render matrix of this rigid body.
    transform: Transform,

    /// The global transform render matrix of this rigid body.
    global_transform: GlobalTransform,
}
