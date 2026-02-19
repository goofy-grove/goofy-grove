use std::pin::Pin;

use gg_core::domain::prelude::*;
use serde_json::json;
use socketioxide::SocketIo;

pub struct PersonCreatedEventHandler {
    socket: SocketIo,
}

impl PersonCreatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<PersonCreatedEvent> for PersonCreatedEventHandler {
    fn handle(&self, event: &PersonCreatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let json = json!({
            "event": "person.created",
            "data": {
                "id": event.person.uid().value(),
                "name": event.person.name().value(),
                "description": event.person.description().value(),
                "creator_id": event.person.creator_id().value(),
            }
        });
        let socket = self.socket.clone();

        Box::pin(async move {
            socket.emit("event", &json).await.ok();
        })
    }
}
