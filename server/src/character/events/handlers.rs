use std::pin::Pin;

use serde_json::json;
use socketioxide::SocketIo;
use tracing::info;

use crate::{
    character::events::types::{
        CharacterCreatedEvent, CharacterDeletedEvent, CharacterUpdatedEvent,
    },
    platform::events::EventHandler,
};

pub struct CharacterCreatedEventHandler {
    socket: SocketIo,
}

impl CharacterCreatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<CharacterCreatedEvent> for CharacterCreatedEventHandler {
    fn handle(&self, event: &CharacterCreatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let creator_uid = event.character.creator_uid.clone();
        let character = event.character.clone();
        let socket = self.socket.clone();
        let exclude_participants = event.exclude_participants.clone();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting character:created event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_uid}"))
                .except(exclude_participants)
                .emit("character:created", &character)
                .await
                .ok();
        })
    }
}

pub struct CharacterUpdatedEventHandler {
    socket: SocketIo,
}

impl CharacterUpdatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<CharacterUpdatedEvent> for CharacterUpdatedEventHandler {
    fn handle(&self, event: &CharacterUpdatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let creator_uid = event.character.creator_uid.clone();
        let character = event.character.clone();
        let socket = self.socket.clone();
        let exclude_participants = event.exclude_participants.clone();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting character:updated event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_uid}"))
                .except(exclude_participants)
                .emit("character:updated", &character)
                .await
                .ok();
        })
    }
}

pub struct CharacterDeletedEventHandler {
    socket: SocketIo,
}

impl CharacterDeletedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<CharacterDeletedEvent> for CharacterDeletedEventHandler {
    fn handle(&self, event: &CharacterDeletedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let creator_uid = event.creator_uid.clone();
        let payload = json!({ "uid": event.character_uid });
        let socket = self.socket.clone();
        let exclude_participants = event.exclude_participants.clone();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting character:deleted event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_uid}"))
                .except(exclude_participants)
                .emit("character:deleted", &payload)
                .await
                .ok();
        })
    }
}
