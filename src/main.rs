//! Awgen is a sandbox game with a heavy emphasis on also acting as a game
//! engine to make smaller mini-games within.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]

mod components;
mod plugins;
mod prefabs;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;


/// The main game app entry function.
fn main() {
    let client = true;
    let mut app = App::new();
    app.add_plugin(plugins::PhysicsPlugin);

    if client {
        app.add_plugin(plugins::WindowPlugin);
        app.add_plugin(plugins::PlayerPlugin);
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_startup_system(prefabs::spawn_basic_scene);
        app.add_startup_system(prefabs::spawn_player);
    }

    app.run();
}
