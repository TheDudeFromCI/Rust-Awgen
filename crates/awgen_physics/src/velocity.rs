//! Contains components and systems for indicating target forces that are
//! applied to an entity in order to move it within the world. This includes
//! both internal and external forces.


use crate::prelude::Position;
use bevy::prelude::*;


/// Indicates that the current entity is capable of generating force to apply
/// to another entity or itself.
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct VelocitySource {
    /// The current amount of force, or velocity, that this velocity source is
    /// generating.
    pub force: Vec3,
}


/// Indicates a moveable entity that may obtain it's velocity from a various
/// number of sources. The total movement about is based on the sum of all
/// forces being provided to this component.
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Movable {
    /// A list of all velocity source providers.
    pub forces: Vec<Entity>,
}


/// Called each physics frame in order to apply velocity to all movable entities
/// and thus update their position.
pub fn apply_velocity(
    mut query: Query<(&mut Position, &Movable)>,
    vel_sources: Query<&VelocitySource>,
) {
    query.par_for_each_mut(32, |(mut position, movable)| {
        let mut force = Vec3::ZERO;
        for velocity_source in &movable.forces {
            force += vel_sources.get(*velocity_source).unwrap().force;
        }

        // TODO: Check for collisions!
        position.translation += force;
    });
}
