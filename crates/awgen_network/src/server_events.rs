//! Contains systems, components, and handlers in charge of distributing player
//! connection events.


use bevy::prelude::*;
use bevy_renet::renet::ServerEvent;


/// A ID pointer that represents a client connection socket.
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ClientSocket {
    /// The Renet client socket ID.
    id: u64,
}

impl ClientSocket {
    /// Creates a new client socket instance with the given client ID.
    pub fn new(id: u64) -> Self {
        Self {
            id,
        }
    }


    /// Gets the client socket ID.
    pub fn id(&self) -> u64 {
        self.id
    }
}


/// An event that is triggered when a new client connects to the server.
pub struct ClientConnectedEvent(Entity);


/// An event that is triggered when a client disconnects from the server.
pub struct ClientDisconnectedEvent(Entity);


/// An event listener that handles when a new client socket is opened or closed.
///
/// This will create new entities with client sockets as needed or dispose them.
/// This will also trigger ClientConnected and ClientDisconnected events for the
/// corresponding entities.
pub fn server_socket_event(
    mut events: EventReader<ServerEvent>,
    mut ev_connected: EventWriter<ClientConnectedEvent>,
    mut ev_disconnected: EventWriter<ClientDisconnectedEvent>,
    mut commands: Commands,
    client_list: Query<(Entity, &ClientSocket)>,
) {
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                let entity = commands.spawn(ClientSocket::new(*id)).id();
                ev_connected.send(ClientConnectedEvent(entity));
            },
            ServerEvent::ClientDisconnected(id) => {
                let (entity, _) = client_list.iter().find(|(_, c)| c.id == *id).unwrap();
                ev_disconnected.send(ClientDisconnectedEvent(entity));
                commands.entity(entity).despawn();
            },
        }
    }
}
