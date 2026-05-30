use std::pin::Pin;

use gg_core::domain::prelude::*;
use serde_json::json;
use socketioxide::SocketIo;
use tracing::info;

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
        let creator_id = event.persona.creator_id.inner().to_owned();
        let json = json!({
            "id": event.persona.uid.inner(),
            "name": event.persona.name.inner(),
            "description": event.persona.description.inner(),
            "creator_id": creator_id,
            "avatar_uid": event.persona.avatar_uid.as_ref().map(|value| value.inner()),
        });
        let socket = self.socket.clone();
        let exclude_participants: Vec<String> = event
            .exclude_participants
            .clone()
            .into_iter()
            .map(|id| id.into_inner())
            .collect();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_id, ?exclude_participants, "Emitting persona:created event");

            socket
                .of("/v1")
                // NOTE: check if it is correct, probably we should handle the None case
                .unwrap()
                .within(format!("user:{creator_id}"))
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
        let creator_id = event.persona.creator_id.inner().to_owned();
        let json = json!({
            "id": event.persona.uid.inner(),
            "name": event.persona.name.inner(),
            "description": event.persona.description.inner(),
            "creator_id": creator_id,
            "avatar_uid": event.persona.avatar_uid.as_ref().map(|value| value.inner()),
        });
        let socket = self.socket.clone();
        let exclude_participants: Vec<String> = event
            .exclude_participants
            .clone()
            .into_iter()
            .map(|id| id.into_inner())
            .collect();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_id, ?exclude_participants, "Emitting persona:updated event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_id}"))
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
        let creator_id = event.persona.creator_id.inner().to_owned();
        let json = json!({
            "id": event.persona.uid.inner(),
        });
        let socket = self.socket.clone();
        let exclude_participants: Vec<String> = event
            .exclude_participants
            .clone()
            .into_iter()
            .map(|id| id.into_inner())
            .collect();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_id, ?exclude_participants, "Emitting persona:deleted event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_id}"))
                .except(exclude_participants)
                .emit("persona:deleted", &json)
                .await
                .ok();
        })
    }
}

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
        let creator_id = event.character.creator_id.inner().to_owned();
        let json = json!({
            "id": event.character.uid.inner(),
            "name": event.character.name.inner(),
            "description": event.character.description.inner(),
            "creator_id": creator_id,
        });
        let socket = self.socket.clone();
        let exclude_participants: Vec<String> = event
            .exclude_participants
            .clone()
            .into_iter()
            .map(|id| id.into_inner())
            .collect();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_id, ?exclude_participants, "Emitting character:created event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_id}"))
                .except(exclude_participants)
                .emit("character:created", &json)
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
        let creator_id = event.character.creator_id.inner().to_owned();
        let json = json!({
            "id": event.character.uid.inner(),
            "name": event.character.name.inner(),
            "description": event.character.description.inner(),
            "creator_id": creator_id,
        });
        let socket = self.socket.clone();
        let exclude_participants: Vec<String> = event
            .exclude_participants
            .clone()
            .into_iter()
            .map(|id| id.into_inner())
            .collect();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_id, ?exclude_participants, "Emitting character:updated event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_id}"))
                .except(exclude_participants)
                .emit("character:updated", &json)
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
        let creator_id = event.character.creator_id.inner().to_owned();
        let json = json!({
            "id": event.character.uid.inner(),
        });
        let socket = self.socket.clone();
        let exclude_participants: Vec<String> = event
            .exclude_participants
            .clone()
            .into_iter()
            .map(|id| id.into_inner())
            .collect();

        Box::pin(async move {
            info!(target: "application::event_bus", ?creator_id, ?exclude_participants, "Emitting character:deleted event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{creator_id}"))
                .except(exclude_participants)
                .emit("character:deleted", &json)
                .await
                .ok();
        })
    }
}

pub struct UserUpdatedEventHandler {
    socket: SocketIo,
}

impl UserUpdatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<UserUpdatedEvent> for UserUpdatedEventHandler {
    fn handle(&self, event: &UserUpdatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let user_id = event.user.uid.inner().to_owned();
        let json = json!({
            "id": user_id,
            "username": event.user.name.inner(),
            "avatar_uid": event.user.avatar_uid.as_ref().map(|value| value.inner().clone()),
        });
        let socket = self.socket.clone();
        let exclude_participants: Vec<String> = event
            .exclude_participants
            .clone()
            .into_iter()
            .map(|id| id.into_inner())
            .collect();

        Box::pin(async move {
            info!(target: "application::event_bus", ?user_id, ?exclude_participants, "Emitting user:updated event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{user_id}"))
                .except(exclude_participants)
                .emit("user:updated", &json)
                .await
                .ok();
        })
    }
}
