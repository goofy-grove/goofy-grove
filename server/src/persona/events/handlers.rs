use std::pin::Pin;

use serde_json::json;
use socketioxide::SocketIo;
use tracing::info;

use crate::{
    persona::events::types::{PersonaCreatedEvent, PersonaDeletedEvent, PersonaUpdatedEvent},
    platform::events::EventHandler,
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
    fn handle(&self, event: &PersonaCreatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let creator_uid = event.persona.creator_uid.clone();
        let json = json!({
            "uid": event.persona.uid,
            "name": event.persona.name,
            "description": event.persona.description,
            "creator_uid": creator_uid,
            "avatar_uid": event.persona.avatar_uid,
        });
        let socket = self.socket.clone();
        let exclude_participants = event.exclude_participants.clone();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting persona:created event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_uid}"))
                .except(exclude_participants)
                .emit("persona:created", &json)
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
    fn handle(&self, event: &PersonaUpdatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let creator_uid = event.persona.creator_uid.clone();
        let json = json!({
            "uid": event.persona.uid,
            "name": event.persona.name,
            "description": event.persona.description,
            "creator_uid": creator_uid,
            "avatar_uid": event.persona.avatar_uid,
        });
        let socket = self.socket.clone();
        let exclude_participants = event.exclude_participants.clone();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting persona:updated event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_uid}"))
                .except(exclude_participants)
                .emit("persona:updated", &json)
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
    fn handle(&self, event: &PersonaDeletedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let creator_uid = event.persona.creator_uid.clone();
        let json = json!({
            "uid": event.persona.uid,
        });
        let socket = self.socket.clone();
        let exclude_participants = event.exclude_participants.clone();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting persona:deleted event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_uid}"))
                .except(exclude_participants)
                .emit("persona:deleted", &json)
                .await
                .ok();
        })
    }
}
