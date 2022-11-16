//! Helper resources for working with physics frame time and render frame
//! interpretation.


use bevy::prelude::*;


/// The number of physics frames that are calculated per second.
#[derive(Debug, Clone, Resource)]
pub struct PhysicsTickrate {
    /// The number of frames per second.
    rate: f32,

    /// The delta time between physics frames, measured in seconds.
    delta: f32,
}

impl PhysicsTickrate {
    /// Creates a new physics tickrate resource for the given number of physics
    /// frames per second.
    pub fn new(rate: f32) -> Self {
        Self {
            rate,
            delta: 1.0 / rate,
        }
    }


    /// Gets the number of physics frames per second.
    pub fn tickrate(&self) -> f32 {
        self.rate
    }


    /// Gets the delta time, in seconds, between physics frames.
    pub fn delta(&self) -> f32 {
        self.delta
    }
}

impl Default for PhysicsTickrate {
    fn default() -> Self {
        PhysicsTickrate::new(20.0)
    }
}


/// A time keeping unit that measures the physics frame time delta for physics
/// rendering interpolation.
#[derive(Debug, Clone, Default, Resource)]
pub struct PhysicsFrame {
    /// The total system time, in seconds, of the last real physics frame.
    last_frame: f32,

    /// The delta percentage between the last physics frame and the next physics
    /// frame.
    delta: f32,

    /// The physics frame number. This value increments by one for every
    /// elapsed physics frame.
    frame_num: u64,
}

impl PhysicsFrame {
    /// Gets the total time, in seconds, of the last physics frame since the
    /// runtime was started.
    pub fn last_frame(&self) -> f32 {
        self.last_frame
    }


    /// Gets the interpolation delta between the last physics frame and the next
    /// physics frame.
    ///
    /// This value is always between 0, inclusive, and 1, exclusive.
    pub fn delta(&self) -> f32 {
        self.delta
    }


    /// Gets the total number of physics frames that have elapsed. This count is
    /// based of the last physics frame, starting at 0.
    pub fn frame_number(&self) -> u64 {
        self.frame_num
    }
}


/// Called every render frame to calculate the physics frame delta for physics
/// interpolation handling.
pub fn update_physics_render_frame(
    time: Res<Time>,
    tickrate: Res<PhysicsTickrate>,
    mut physics: ResMut<PhysicsFrame>,
) {
    let cur_frame = time.elapsed_seconds();
    let progress = (cur_frame - physics.last_frame) / tickrate.delta();
    physics.delta = num::clamp(progress as f32, 0.0, 1.0);
}


/// Called at the beginning of a physics frame to prepare the timing for
/// calculating physics render frames.
pub fn prepare_physics_render_frame(time: Res<Time>, mut frame: ResMut<PhysicsFrame>) {
    frame.last_frame = time.elapsed_seconds();
    frame.frame_num += 1;
}
