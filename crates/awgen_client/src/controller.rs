//! The controller and user input handling components and systems.


use awgen_physics::prelude::VelocitySource;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::f32::consts::PI;


/// A component marker that allows for an entity to supply a velocity force
/// based off of WASD input controls.
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct WasdController;


/// A component that reads a continuous euler rotation based off of mouse
/// movement inputs.
#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct MouseController {
    /// The mouse sensitivity of this camera controller.
    pub sensitivity: f32,

    /// Whether or not the mouse is currently locked to the window.
    pub locked: bool,

    /// The current euler angle of the mouse input.
    pub angle: Vec3,
}

impl MouseController {
    /// Gets the quaternion value of the currently represented mouse rotation.
    pub fn quat(&self) -> Quat {
        Quat::from_euler(EulerRot::YXZ, self.angle.y, self.angle.x, self.angle.z)
    }
}

impl Default for MouseController {
    fn default() -> Self {
        Self {
            sensitivity: 0.6,
            locked:      false,
            angle:       Vec3::ZERO,
        }
    }
}


/// A system that is triggered every physics frame in order to update the
/// velocity source of a WASD-controlled entity.
pub fn wasd_velocity_input(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut VelocitySource, &MouseController), With<WasdController>>,
) {
    for (mut source, controller) in query.iter_mut() {
        source.force = Vec3::ZERO;

        if keyboard.pressed(KeyCode::W) {
            source.force += Vec3::NEG_Z;
        }

        if keyboard.pressed(KeyCode::A) {
            source.force += Vec3::NEG_X;
        }

        if keyboard.pressed(KeyCode::S) {
            source.force += Vec3::Z;
        }

        if keyboard.pressed(KeyCode::D) {
            source.force += Vec3::X;
        }

        if source.force.length_squared() > 0.0 {
            source.force = controller.quat() * source.force;
            source.force.y = 0.0;
            source.force = source.force.normalize();
        }
    }
}


/// Updates the look rotation of all mouse controller components.
pub fn mouse_rotation_input(
    mut mouse: EventReader<MouseMotion>,
    mut query: Query<&mut MouseController>,
) {
    let mut rotation = Vec2::ZERO;
    mouse.iter().for_each(|ev| {
        rotation += ev.delta;
    });

    for mut controller in query.iter_mut() {
        rotation *= controller.sensitivity;

        if rotation.length_squared() <= 0.0 || !controller.locked {
            return;
        }

        controller.angle.x -= rotation.y * PI * 0.001;
        controller.angle.y -= rotation.x * PI * 0.001;

        controller.angle.x = num::clamp(controller.angle.x, -1.55, 1.55);
        controller.angle.y %= PI * 2.0;
    }
}


/// Toggles whether or not the cursor is locked within the screen bounds each
/// time the F11 key is pressed.
pub fn toggle_cursor(
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut query: Query<&mut MouseController>,
) {
    let window = windows.get_primary_mut().unwrap();
    for mut camera in query.iter_mut() {
        if input.just_pressed(KeyCode::F11) {
            camera.locked = !camera.locked;

            window.set_cursor_lock_mode(camera.locked);
            window.set_cursor_visibility(!camera.locked);
        }
    }
}