//! The camera controller plugin for Awgen


use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::f32::consts::PI;


/// The implementation for the camera controller plugin.
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FirstPersonCamera>()
            .add_startup_system(spawn_camera)
            .add_system(update_camera_rotation)
            .add_system(toggle_cursor);
    }
}


/// A FirstPersonCamera controller component.
#[derive(Component, Reflect)]
pub struct FirstPersonCamera {
    /// The look angle of this camera in radian euler coordinates.
    pub angle: Vec3,

    /// The mouse sensitivity of this camera controller.
    pub sensitivity: f32,

    /// Whether or not the mouse is currently locked to the window screen.
    pub mouse_locked: bool,
}

impl Default for FirstPersonCamera {
    fn default() -> Self {
        FirstPersonCamera {
            angle:        Vec3::ZERO,
            sensitivity:  0.6,
            mouse_locked: false,
        }
    }
}


/// Updates the look rotation of the first person camera.
fn update_camera_rotation(
    mut mouse: EventReader<MouseMotion>, mut query: Query<(&mut FirstPersonCamera, &mut Transform)>,
) {
    let mut rotation = Vec2::ZERO;
    mouse.iter().for_each(|ev| {
        rotation += ev.delta;
    });

    for (mut camera, mut transform) in query.iter_mut() {
        rotation *= camera.sensitivity;

        if rotation.length_squared() <= 0.0 || !camera.mouse_locked {
            return;
        }

        camera.angle.x -= rotation.y * PI * 0.001;
        camera.angle.y -= rotation.x * PI * 0.001;

        camera.angle.x = num::clamp(camera.angle.x, -1.55, 1.55);
        camera.angle.y %= PI * 2.0;

        transform.rotation = Quat::from_axis_angle(Vec3::Z, camera.angle.z)
            * Quat::from_axis_angle(Vec3::Y, camera.angle.y)
            * Quat::from_axis_angle(Vec3::X, camera.angle.x);
    }
}


/// Spawns a 3D projection camera and adds it to the scene.
fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0),
            ..default()
        })
        .insert(FirstPersonCamera::default());
}


/// Toggles whether or not the cursor is locked within the screen bounds each
/// time the F11 key is pressed.
fn toggle_cursor(
    input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>,
    mut query: Query<&mut FirstPersonCamera>,
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
