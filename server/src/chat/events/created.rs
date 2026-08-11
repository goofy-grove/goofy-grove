use std::pin::Pin;

use socketioxide::SocketIo;
use tracing::info;

use crate::{
    chat::db::Chat,
    platform::events::{Event, EventHandler},
};

#[derive(Debug, Clone)]
pub struct ChatCreatedEvent {
    pub chat: Chat,
    pub exclude_participants: Vec<String>,
}

impl Event for ChatCreatedEvent {}

pub struct ChatCreatedEventHandler {
    socket: SocketIo,
}

impl ChatCreatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<ChatCreatedEvent> for ChatCreatedEventHandler {
    fn handle(&self, event: ChatCreatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let rooms: Vec<String> = event
                .chat
                .members
                .iter()
                .map(|member| format!("user:{}", member.user.uid))
                .collect();

            socket
                .clone()
                .of("/v1")
                .unwrap()
                .within(rooms.clone())
                .join(format!("chat:{}", event.chat.uid))
                .await
                .unwrap();

            let creator_uid = event.chat.creator_uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?creator_uid, ?exclude_participants, "Emitting chat:created event");

            socket
                .of("/v1")
                .unwrap()
                .within(rooms)
                .except(exclude_participants)
                .emit("chat:created", &event.chat)
                .await
                .ok();
        })
    }
}
