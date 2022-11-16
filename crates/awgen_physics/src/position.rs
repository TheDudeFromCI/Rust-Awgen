//! Components and systems related to managing an entity's position and movement
//! vectors.


use crate::PhysicsFrame;
use bevy::prelude::*;


/// The absolute position of an entity on a physics frame.
#[derive(Debug, Clone, Reflect, Component)]
// #[reflect(Component)]
pub struct Position {
    /// The translation value of the entity within the world, measured in
    /// meters.
    pub translation: Vec3,

    /// The rotation value of the entity within the world.
    pub rotation: Quat,

    /// The scale value of the entity within the world.
    pub scale: Vec3,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation:    Quat::IDENTITY,
            scale:       Vec3::ONE,
        }
    }
}


/// The value of an entity's position on the previous physics frame.
#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct PreviousPosition {
    /// The translation value of the entity within the world, measured in
    /// meters.
    pub translation: Vec3,

    /// The rotation value of the entity within the world.
    pub rotation: Quat,

    /// The scale value of the entity within the world.
    pub scale: Vec3,
}

impl Default for PreviousPosition {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation:    Quat::IDENTITY,
            scale:       Vec3::ONE,
        }
    }
}

/// Updates the render position of the entity between physics frames for
/// smoother movement interpolation.
pub fn update_render_position(
    frame: Res<PhysicsFrame>,
    mut query: Query<(&mut Transform, &Position, &PreviousPosition)>,
) {
    let delta = frame.delta();
    query.par_for_each_mut(128, move |(mut transform, next, last)| {
        transform.translation = last.translation.lerp(next.translation, delta);
        transform.rotation = last.rotation.slerp(next.rotation, delta);
        transform.scale = last.scale.lerp(next.scale, delta);
    });
}


/// This is called once at the beginning of each physics frame to assign the
/// current entity position as the last render frame position.
///
/// This allows for the calculation of render position calculations.
pub fn push_position_stack(mut query: Query<(&mut PreviousPosition, &Position)>) {
    query.par_for_each_mut(512, |(mut previous, pos)| {
        previous.translation = pos.translation;
        previous.rotation = pos.rotation;
        previous.scale = pos.scale;
    });
}
