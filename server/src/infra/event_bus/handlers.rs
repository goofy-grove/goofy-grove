use std::pin::Pin;

use gg_core::domain::prelude::*;
use serde_json::json;
use socketioxide::SocketIo;

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
        let json = json!({
            "event": "persona.created",
            "data": {
                "id": event.persona.uid().value(),
                "name": event.persona.name().value(),
                "description": event.persona.description().value(),
                "creator_id": event.persona.creator_id().value(),
            }
        });
        let socket = self.socket.clone();

        Box::pin(async move {
            socket.emit("event", &json).await.ok();
        })
    }
}
