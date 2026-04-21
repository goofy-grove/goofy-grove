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
        let creator_id = event.persona.creator_id().inner().to_owned();
        let json = json!({
            "id": event.persona.uid().inner(),
            "name": event.persona.name().inner(),
            "description": event.persona.description().inner(),
            "creator_id": creator_id,
        });
        let socket = self.socket.clone();

        let exclude_participants: Vec<String> = event
            .exclude_participants
            .clone()
            .into_iter()
            .map(|v| v.inner().to_owned())
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
