//! Awgen is a sandbox game with a heavy emphasis on also acting as a game
//! engine to make smaller mini-games within.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


mod prefabs;

use awgen_client::ClientPlugin;
use awgen_network::NetworkPlugin;
use awgen_physics::PhysicsPlugin;
use awgen_server::ServerPlugin;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use clap::{Parser, Subcommand};
use std::panic;


/// The default window title for the Awgen game engine.
const WINDOW_TITLE: &str = "Awgen";


/// The default background clear color.
const CLEAR_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);


/// The number of physics frames to calculate per second.
const TICKRATE: f32 = 25.0;


/// The maximum of clients that can connect to a server at once.
const MAX_CLIENTS: usize = 128;


/// The command line input argument structure.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable debug mode
    #[arg(long)]
    debug: bool,

    /// Type of network application to launch.
    #[command(subcommand)]
    network_command: NetworkCommand,
}


/// The command line input argument subcommand structure.
#[derive(Debug, Subcommand)]
enum NetworkCommand {
    /// Launch a new Awgen client instance and joins a server.
    Client {
        /// The IP of the server to join.
        ip: String,

        /// The port of the server to join.
        port: u16,
    },

    /// Launches a new Awgen server instance.
    Server {
        /// The port to open the server on.
        port: u16,
    },

    /// Launch a private server and connect to it in single player mode.
    Localhost,
}


/// The main game app entry function.
fn main() {
    let cli = Cli::parse();
    let debug = cli.debug;

    match cli.network_command {
        NetworkCommand::Client {
            ip,
            port,
        } => launch_client(ip, port, debug),

        NetworkCommand::Server {
            port,
        } => launch_server(port, debug),

        NetworkCommand::Localhost => {
            let port = 3000;
            let ip = "127.0.0.1".to_string();

            let server_thread = std::thread::Builder::new()
                .name("Server".to_string())
                .spawn(move || launch_server(port, debug))
                .unwrap();

            let client_panic = panic::catch_unwind(move || launch_client(ip, port, debug));
            let server_panic = server_thread.join();
            client_panic.and(server_panic).unwrap();
        },
    }
}


/// Launches a new Awgen client instance.
fn launch_client(ip: String, port: u16, debug: bool) {
    let window_title = match debug {
        true => WINDOW_TITLE.to_string(),
        false => format!("{WINDOW_TITLE} [Debug]"),
    };

    let client = match debug {
        true => ClientPlugin::debug(),
        false => ClientPlugin::default(),
    };

    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: window_title,
                        ..default()
                    },
                    ..default()
                })
                .set(log_plugin(debug))
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(PhysicsPlugin::new(TICKRATE))
        .add_plugin(NetworkPlugin::new_client(ip, port))
        .add_plugin(client)
        .add_startup_system(prefabs::spawn_basic_scene)
        .add_startup_system(prefabs::spawn_player)
        .run();
}


/// Launches a new Awgen server instance.
fn launch_server(port: u16, debug: bool) {
    let server = match debug {
        true => ServerPlugin::debug(),
        false => ServerPlugin::default(),
    };

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(log_plugin(debug))
        .add_plugin(PhysicsPlugin::new(TICKRATE))
        .add_plugin(NetworkPlugin::new_server(port, MAX_CLIENTS))
        .add_plugin(server)
        .run();
}


/// Configures the logging plugin based on whether the application is launched
/// in debug mode or not.
fn log_plugin(debug: bool) -> LogPlugin {
    match debug {
        true => {
            LogPlugin {
                level:  Level::DEBUG,
                filter: "info,wgpu=error,awgen=debug".to_string(),
            }
        },
        false => {
            LogPlugin {
                level:  Level::INFO,
                filter: "info,wgpu=error".to_string(),
            }
        },
    }
}
