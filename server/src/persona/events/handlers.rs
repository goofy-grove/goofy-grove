use std::pin::Pin;

use serde_json::json;
use socketioxide::SocketIo;
use tracing::info;

use crate::{
    persona::events::types::{PersonaCreatedEvent, PersonaDeletedEvent, PersonaUpdatedEvent},
    platform::{events::EventHandler, socketio::SOCKET_NAMESPACE},
};

pub struct PersonaCreatedEventHandler {
    socket: SocketIo,
}

impl PersonaCreatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<PersonaCreatedEvent> for PersonaCreatedEventHandler {
    fn handle(&self, event: PersonaCreatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let creator_uid = event.persona.creator_uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting persona:created event");

            socket
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("user:{creator_uid}"))
                .except(exclude_participants)
                .emit("persona:created", &event.persona)
                .await
                .ok();
        })
    }
}

pub struct PersonaUpdatedEventHandler {
    socket: SocketIo,
}

impl PersonaUpdatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<PersonaUpdatedEvent> for PersonaUpdatedEventHandler {
    fn handle(&self, event: PersonaUpdatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let creator_uid = event.persona.creator_uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting persona:updated event");

            socket
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("user:{creator_uid}"))
                .except(exclude_participants)
                .emit("persona:updated", &event.persona)
                .await
                .ok();
        })
    }
}

pub struct PersonaDeletedEventHandler {
    socket: SocketIo,
}

impl PersonaDeletedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<PersonaDeletedEvent> for PersonaDeletedEventHandler {
    fn handle(&self, event: PersonaDeletedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let payload = json!({ "uid": event.persona.uid });
        let socket = self.socket.clone();

        Box::pin(async move {
            let creator_uid = event.persona.creator_uid;
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting persona:deleted event");

            socket
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("user:{creator_uid}"))
                .except(exclude_participants)
                .emit("persona:deleted", &payload)
                .await
                .ok();
        })
    }
}
