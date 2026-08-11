use std::pin::Pin;

use serde_json::json;
use socketioxide::SocketIo;
use tracing::info;

use crate::platform::events::{Event, EventHandler};

#[derive(Debug, Clone)]
pub struct ChatDeletedEvent {
    pub chat_uid: String,
    pub exclude_participants: Vec<String>,
}

impl Event for ChatDeletedEvent {}

pub struct ChatDeletedEventHandler {
    socket: SocketIo,
}

impl ChatDeletedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<ChatDeletedEvent> for ChatDeletedEventHandler {
    fn handle(&self, event: ChatDeletedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let payload = json!({ "uid": event.chat_uid });
        let socket = self.socket.clone();

        Box::pin(async move {
            let chat_uid = event.chat_uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?chat_uid, ?exclude_participants, "Emitting chat:deleted event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("chat:{}", event.chat_uid))
                .except(exclude_participants)
                .emit("chat:deleted", &payload)
                .await
                .ok();
        })
    }
}
