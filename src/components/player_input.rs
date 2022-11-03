//! A set of player-input-related components.


use bevy::prelude::*;


/// A FirstPersonCamera controller component.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FirstPersonController {
    /// The mouse sensitivity of this camera controller.
    pub sensitivity: f32,

    /// Whether or not the mouse is currently locked to the window screen.
    pub mouse_locked: bool,
}

impl Default for FirstPersonController {
    fn default() -> Self {
        Self {
            sensitivity:  0.6,
            mouse_locked: false,
        }
    }
}


/// A component that allows for an entity to be controlled using WASD input
/// keys.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct WasdController;
