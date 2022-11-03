//! A set of movement-related components.


use bevy::prelude::*;


/// Contains a local-space euler coordinate vector indicating the direction the
/// direction that an entity is looking in.
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct LookDirection {
    /// The euler coordinates of the angle, in radians.
    pub angle: Vec3,
}

impl LookDirection {
    /// Creates a new quaternion rotation value from this look direction.
    pub fn quat(&self) -> Quat {
        Quat::from_euler(EulerRot::YXZ, self.angle.y, self.angle.x, self.angle.z)
    }
}


/// Contains movement speed information speed for mobile entities.
#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct MovementSpeed {
    /// The movement speed value, measured in meters per second.
    pub value: f32,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        Self {
            value: 3.0,
        }
    }
}


/// Contains a directional input vector to indicate which direction the mob is
/// attempting to move in. This is measured in local space relative to the
/// direction that the mob is facing as defined by the mob's local transform or
/// look direction, as needed.
#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct MovementInput {
    /// The local input vector value.
    pub value: Vec3,

    /// If true, the forward vector is taken from the mob's looking direction.
    /// If false, the forward vector is taken from the mob's local transform
    /// instead. If the mob does not have a look direction component, this
    /// value is always assumed to be false.
    pub from_look: bool,

    /// Whether or not this component is applied in all 3 directions. If true,
    /// the Y axis is considered when applying the movement transform.
    /// Otherwise, only the X and Z axises are considered.
    pub can_fly: bool,
}

impl MovementInput {
    /// Creates a new movement input that is based off of the entity's look
    /// direction.
    pub fn from_look() -> Self {
        Self {
            value:     Vec3::ZERO,
            from_look: true,
            can_fly:   false,
        }
    }


    /// Gets whether or not the mob is currently moving.
    pub fn is_moving(&self) -> bool {
        self.value.length_squared() > 0.0
    }
}

impl Default for MovementInput {
    fn default() -> Self {
        Self {
            value:     Vec3::ZERO,
            from_look: false,
            can_fly:   false,
        }
    }
}


/// A component that indicates that the transform of this entity should be coped
/// from the look direction of another entity.
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CopyFromLookDir {
    /// The entity that has the look direction component to copy the rotation
    /// from.
    pub entity: Option<Entity>,
}

impl CopyFromLookDir {
    /// Creates a new CopyFromLookDir instance, based on the given input entity
    /// target.
    pub fn from(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
        }
    }
}
