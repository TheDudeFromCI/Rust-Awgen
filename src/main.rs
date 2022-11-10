//! Awgen is a sandbox game with a heavy emphasis on also acting as a game
//! engine to make smaller mini-games within.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


// mod prefabs;

use awgen_client::ClientPlugin;
use awgen_network::NetworkPlugin;
use awgen_physics::PhysicsPlugin;
use awgen_server::ServerPlugin;
use bevy::prelude::*;
use clap::{Parser, Subcommand};
use std::panic;


/// The command line input argument structure.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Type of network application to launch.
    #[command(subcommand)]
    network_command: NetworkCommand,

    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,
}


/// The command line input argument subcommand structure.
#[derive(Subcommand)]
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
        } => launch_server(port),
        NetworkCommand::Localhost => {
            let port = 3000;
            let ip = "127.0.0.1".to_string();

            let server_thread = std::thread::Builder::new()
                .name("Server".to_string())
                .spawn(move || launch_server(port))
                .unwrap();

            let client_panic = panic::catch_unwind(move || launch_client(ip, port, debug));
            let server_panic = server_thread.join();
            client_panic.and(server_panic).unwrap();
        },
    }
}


/// Launches a new Awgen client instance.
fn launch_client(ip: String, port: u16, debug: bool) {
    let client = match debug {
        true => ClientPlugin::debug(),
        false => ClientPlugin::default(),
    };

    App::new()
        .add_plugin(client)
        .add_plugin(PhysicsPlugin::new(20.0))
        .add_plugin(NetworkPlugin::new_client(ip, port))
        .run();
}


/// Launches a new Awgen server instance.
fn launch_server(port: u16) {
    App::new()
        .add_plugin(ServerPlugin::default())
        .add_plugin(PhysicsPlugin::new(20.0))
        .add_plugin(NetworkPlugin::new_server(port, 128))
        .run();
}
