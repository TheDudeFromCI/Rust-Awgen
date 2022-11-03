//! Systems related to the movement, collision, and similar physics of moving
//! entities.


use crate::components::{self, CopyFromLookDir, LookDirection};
use bevy::prelude::*;


/// Applies a translation offset to a mob's local transform based off it's
/// movement input vector and movement speed.
pub fn apply_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &components::MovementInput,
        &components::MovementSpeed,
        Option<&components::LookDirection>,
    )>,
) {
    query.par_for_each_mut(64, |(mut transform, input, speed, look_direction)| {
        if !input.is_moving() {
            return;
        }

        let rot = match (input.from_look, look_direction) {
            (true, Some(euler)) => euler.quat(),
            _ => transform.rotation,
        };

        let mut shift = rot * input.value;
        shift = flatten_vec(shift);
        shift *= time.delta().as_secs_f32() * speed.value;

        validate_collision(&transform, &mut shift);
        transform.translation += shift;
    });
}


/// Modifies the current shift magnitude based off what is allowed by the
/// physics engine.
fn validate_collision(_transform: &Transform, _shift: &mut Vec3) {
    // TODO: Proper collision detection
}


/// Flattens a vector along the Y axis and normalizes the resulting 2D vector.
fn flatten_vec(mut vec: Vec3) -> Vec3 {
    vec.y = 0.0;
    vec.normalize()
}


/// Causes entities with the CopyFromLookDir component to copy their transform
/// rotational value from another entity containing a LookDirection component.
pub fn copy_transform_from_look_dir(
    mut query: Query<(&mut Transform, &CopyFromLookDir)>,
    lookers: Query<&LookDirection>,
) {
    for (mut transform, copy_target) in query.iter_mut() {
        if let Some(entity) = copy_target.entity {
            if let Ok(look_dir) = lookers.get(entity) {
                transform.rotation = look_dir.quat();
            }
        }
    }
}
