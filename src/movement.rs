//! The first person movement controller plugin for Awgen.


use crate::camera::FirstPersonCamera;
use bevy::prelude::*;


/// The first person movement controller plugin implementation.
pub struct FirstPersonMovementPlugin;

impl Plugin for FirstPersonMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WasdMovement>()
            .add_system(add_movement_to_player)
            .add_system(wasd_movement);
    }
}


/// A component that allows for an entity to travel along a horizontal axis at a
/// set movement speed.
#[derive(Reflect, Component)]
pub struct WasdMovement {
    /// The entity's movement speed.
    pub movement_speed: f32,
}

impl Default for WasdMovement {
    fn default() -> Self {
        WasdMovement {
            movement_speed: 8.0,
        }
    }
}


/// Allows for an entity to move around when using the WASD keys on the
/// keyboard.
fn wasd_movement(
    time: Res<Time>, keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&WasdMovement, &mut Transform)>,
) {
    for (movement, mut transform) in query.iter_mut() {
        let forward = flatten(transform.forward());
        let right = flatten(transform.right());
        let mut shift = Vec3::ZERO;

        if keyboard.pressed(KeyCode::W) {
            shift += forward;
        }

        if keyboard.pressed(KeyCode::A) {
            shift += -right;
        }

        if keyboard.pressed(KeyCode::S) {
            shift += -forward;
        }

        if keyboard.pressed(KeyCode::D) {
            shift += right;
        }

        if shift.length_squared() <= 0.0 {
            continue;
        }

        shift = shift.normalize();
        shift *= movement.movement_speed * time.delta().as_secs_f32();
        transform.translation += shift;
    }
}


/// Flattens a directional vector along the XZ plane, and normalizes it.
fn flatten(mut vec: Vec3) -> Vec3 {
    vec.y = 0.0;
    vec.normalize()
}


/// Adds WASD movement support for the first person player camera.
fn add_movement_to_player(mut commands: Commands, query: Query<Entity, Added<FirstPersonCamera>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(WasdMovement::default());
    }
}
