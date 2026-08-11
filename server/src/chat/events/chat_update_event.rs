use std::pin::Pin;

use socketioxide::SocketIo;
use tracing::info;

use crate::{
    chat::db::chat::Chat,
    platform::events::{Event, EventHandler},
};

#[derive(Debug, Clone)]
pub struct ChatUpdatedEvent {
    pub chat: Chat,
    pub exclude_participants: Vec<String>,
}

impl Event for ChatUpdatedEvent {}

pub struct ChatUpdatedEventHandler {
    socket: SocketIo,
}

impl ChatUpdatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<ChatUpdatedEvent> for ChatUpdatedEventHandler {
    fn handle(&self, event: ChatUpdatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let creator_uid = event.chat.creator_uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting chat:updated event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("chat:{}", event.chat.uid))
                .except(exclude_participants)
                .emit("chat:updated", &event.chat)
                .await
                .ok();
        })
    }
}
