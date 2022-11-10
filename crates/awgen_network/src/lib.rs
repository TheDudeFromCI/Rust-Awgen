//! The networking implementation for the Awgen game engine.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


pub mod server_events;


/// A re-export of all components and systems defined within this crate.
pub mod prelude {
    pub use super::server_events::*;
    pub use super::*;
}


use bevy::prelude::*;
use bevy_renet::renet::{
    ClientAuthentication, RenetClient, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig
};
use bevy_renet::{RenetClientPlugin, RenetServerPlugin};
use prelude::*;
use std::net::UdpSocket;
use std::time::SystemTime;


/// The current networking protocol index for this version of the Awgen
/// networking plugin.
const PROTOCOL_ID: u64 = 1;


/// An indicator for the side of the network to be handled within the runtime.
pub enum NetworkSide {
    /// The client-side of the network.
    Client {
        /// The ip of the server to connect to.
        ip: String,

        /// The port of the server to connect to.
        port: u16,
    },

    /// The server-side of the network.
    Server {
        /// The port to start the server on.
        port: u16,

        /// The maximum number of clients that are allowed on the server at
        /// once.
        max_clients: usize,
    },
}


/// The implementation of the Awgen networking plugin.
pub struct NetworkPlugin {
    /// The side of the network begin handled.
    side: NetworkSide,
}

impl NetworkPlugin {
    /// Creates a new server instance of the network plugin.
    pub fn new_server(port: u16, max_clients: usize) -> Self {
        Self {
            side: NetworkSide::Server {
                port,
                max_clients,
            },
        }
    }


    /// Creates a new client instance of the network plugin.
    pub fn new_client<S>(ip: S, port: u16) -> Self
    where S: Into<String> {
        Self {
            side: NetworkSide::Client {
                ip: ip.into(),
                port,
            },
        }
    }


    /// Gets the side of the network currently being represented.
    pub fn get_side(&self) -> &NetworkSide {
        &self.side
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        match &self.side {
            NetworkSide::Server {
                port,
                max_clients,
            } => {
                app.add_plugin(RenetServerPlugin)
                    .insert_resource(build_server(*port, *max_clients))
                    .register_type::<ClientSocket>()
                    .add_event::<ClientConnectedEvent>()
                    .add_event::<ClientDisconnectedEvent>()
                    .add_system(server_socket_event)
            },
            NetworkSide::Client {
                ip,
                port,
            } => app.add_plugin(RenetClientPlugin).insert_resource(build_client(ip, *port)),
        };
    }
}


/// Builds a new Renet Server instance on the given port.
fn build_server(port: u16, max_clients: usize) -> RenetServer {
    let server_addr = format!("127.0.0.1:{port}").parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let auth = ServerAuthentication::Unsecure;
    let server_config = ServerConfig::new(max_clients, PROTOCOL_ID, server_addr, auth);
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    RenetServer::new(time, server_config, connection_config, socket).unwrap()
}


/// Builds a new Renet Client instance on the given port.
fn build_client(ip: &str, port: u16) -> RenetClient {
    let server_addr = format!("{ip}:{port}").parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = time.as_millis() as u64;
    let auth = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    RenetClient::new(time, socket, client_id, connection_config, auth).unwrap()
}
