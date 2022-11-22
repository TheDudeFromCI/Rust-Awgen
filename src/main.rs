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
use awgen_world::WorldDataPlugin;
use awgen_world_mesh::WorldMeshPlugin;
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


/// The error string format for the Awgen server and client threads.
macro_rules! print_error {
    ( $msg:expr, $err:expr ) => {
        println!(
            "\n===== {{ ERROR }} =====\n{0}\nError: {1:?}\n=====================\n",
            $msg, $err
        );
    };
}


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
        NetworkCommand::Localhost => launch_localhost(debug),
    }
}


/// Launches a new localhost Awgen server and a client instance that connects to
/// it.
fn launch_localhost(debug: bool) {
    let port = 30082;
    let ip = "127.0.0.1".to_string();

    let server_thread = std::thread::Builder::new()
        .name("Server".to_string())
        .spawn(move || launch_server(port, debug))
        .unwrap();

    launch_client(ip, port, debug);
    server_thread.join().unwrap();
}


/// Launches a new Awgen client instance.
fn launch_client(ip: String, port: u16, debug: bool) {
    let result = panic::catch_unwind(move || {
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
                    .set(LogPlugin {
                        level: Level::WARN,
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
            )
            .add_plugin(PhysicsPlugin::new(TICKRATE))
            .add_plugin(NetworkPlugin::new_client(ip, port))
            .add_plugin(WorldDataPlugin::default())
            .add_plugin(WorldMeshPlugin::default())
            .add_plugin(client)
            .add_startup_system(prefabs::spawn_basic_scene)
            .add_startup_system(prefabs::spawn_player)
            .run();
    });

    if let Err(err) = result {
        print_error!("An internal error has occurred in the Awgen client.", err);
    }
}


/// Launches a new Awgen server instance.
fn launch_server(port: u16, debug: bool) {
    let result = panic::catch_unwind(move || {
        let server = match debug {
            true => ServerPlugin::debug(),
            false => ServerPlugin::default(),
        };

        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugin(PhysicsPlugin::new(TICKRATE))
            .add_plugin(NetworkPlugin::new_server(port, MAX_CLIENTS))
            .add_plugin(WorldDataPlugin::default())
            .add_plugin(server)
            .run();
    });

    if let Err(err) = result {
        print_error!("An internal error has occurred in the Awgen server.", err);
    }
}
