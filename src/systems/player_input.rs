//! Systems related to player input and controls.


use crate::components::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::f32::consts::PI;


/// Updates the look rotation of the first person camera.
pub fn update_camera_rotation(
    mut mouse: EventReader<MouseMotion>,
    mut query: Query<(&mut LookDirection, &FirstPersonController)>,
) {
    let mut rotation = Vec2::ZERO;
    mouse.iter().for_each(|ev| {
        rotation += ev.delta;
    });

    for (mut look_dir, camera) in query.iter_mut() {
        rotation *= camera.sensitivity;

        if rotation.length_squared() <= 0.0 || !camera.mouse_locked {
            return;
        }

        look_dir.angle.x -= rotation.y * PI * 0.001;
        look_dir.angle.y -= rotation.x * PI * 0.001;

        look_dir.angle.x = num::clamp(look_dir.angle.x, -1.55, 1.55);
        look_dir.angle.y %= PI * 2.0;
    }
}


/// Toggles whether or not the cursor is locked within the screen bounds each
/// time the F11 key is pressed.
pub fn toggle_cursor(
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut query: Query<&mut FirstPersonController>,
) {
    let mut camera = query.single_mut();
    let window = windows.get_primary_mut().unwrap();

    if input.just_pressed(KeyCode::F11) {
        let locked = !camera.mouse_locked;
        camera.mouse_locked = locked;

        window.set_cursor_lock_mode(locked);
        window.set_cursor_visibility(!locked);
    }
}


/// Allows for an entity to move around when using the WASD keys on the
/// keyboard.
pub fn wasd_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut MovementInput, With<WasdController>>,
) {
    for mut input in query.iter_mut() {
        input.value = Vec3::ZERO;

        if keyboard.pressed(KeyCode::W) {
            input.value += Vec3::NEG_Z;
        }

        if keyboard.pressed(KeyCode::A) {
            input.value += Vec3::NEG_X;
        }

        if keyboard.pressed(KeyCode::S) {
            input.value += Vec3::Z;
        }

        if keyboard.pressed(KeyCode::D) {
            input.value += Vec3::X;
        }

        if input.value.length_squared() > 0.0 {
            input.value = input.value.normalize();
        }
    }
}
